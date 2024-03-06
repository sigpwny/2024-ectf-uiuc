pub use max78000_pac::I2C1;

pub enum MasterI2CStatus {
    Success = 0,
    Nack = -1,
    InvalidState = -2,
    ArbitrationLost = -3,
}

/// Configure I2C master
pub fn master_config(i2c1: &I2C1) {
    // Enable I2C peripheral
    i2c1.ctrl().modify(|_,w| w.en().set_bit());
    while i2c1.ctrl().read().en().bit_is_clear() { }
    // Set master mode
    i2c1.ctrl().modify(|_, w| w.mst_mode().set_bit());
    while i2c1.ctrl().read().mst_mode().bit_is_clear() { }
    // Clear FIFOs
    clear_txfifo(&i2c1);
    clear_rxfifo(&i2c1);
    // Set frequency
    set_frequency(&i2c1, 100_000);
}

/// Configure I2C slave
pub fn slave_config(i2c1: &I2C1, addr: u8) {
    let addr: u8 = addr & 0x7F;
    // Disable I2C peripheral
    i2c1.ctrl().modify(|_, w| w.en().clear_bit());
    while i2c1.ctrl().read().en().bit_is_set() { }
    // Configure slave mode
    i2c1.ctrl().modify(|_, w| w
        .mst_mode().clear_bit()     // Disable master mode
        .gc_addr_en().clear_bit()   // Disable general call
        .irxm_en().clear_bit()      // Disable IRXM
        .clkstr_dis().clear_bit()   // Enable clock stretching
    );
    // Configure just-in-time
    i2c1.txctrl0().modify(|_, w| w
        .nack_flush_dis().set_bit()
        .rd_addr_flush_dis().clear_bit()
        .wr_addr_flush_dis().clear_bit()
        .gc_addr_flush_dis().clear_bit()
        .preload_mode().clear_bit()
    );
    // Set slave address
    i2c1.slave0().write(|w| unsafe {
        w.bits(addr as u32)
    });
    // Clear FIFOs
    clear_txfifo(&i2c1);
    clear_rxfifo(&i2c1);
    // Set frequency
    set_frequency(&i2c1, 100_000);
    // Enable I2C peripheral
    i2c1.ctrl().modify(|_, w| w.en().set_bit());
    while i2c1.ctrl().read().en().bit_is_clear() { }
}

/// Set I2C frequency
fn set_frequency(i2c1: &I2C1, hz: u32){
    let ticks_total: u16 = (50_000_000 / hz) as u16;
    let hi_clks: u16 = (ticks_total >> 1) - 1;
    let low_clks: u16 = (ticks_total >> 1) - 1;
    i2c1.clkhi().modify( unsafe{|_,w| w.hi().bits(hi_clks)});
    i2c1.clklo().modify( unsafe{|_,w| w.lo().bits(low_clks)});
}

/// Clear TX FIFO
fn clear_txfifo(i2c1: &I2C1){
    i2c1.txctrl0().modify(|_,w| w.flush().flush());
    while i2c1.txctrl0().read().flush().is_flush() {}
}
/// Clear RX FIFO
fn clear_rxfifo(i2c1: &I2C1){
    i2c1.rxctrl0().modify(|_,w| w.flush().flush());
    while i2c1.rxctrl0().read().flush().is_flush() {}
}
/// Check if RX FIFO is empty
fn rx_em(i2c1: &I2C1) -> bool {
    return i2c1.status().read().rx_em().bit_is_set();
}
/// Check if RX FIFO is full
fn rx_full(i2c1: &I2C1) -> bool {
    return i2c1.status().read().rx_full().bit_is_set();
}
/// Check if TX FIFO is empty
fn tx_em(i2c1: &I2C1) -> bool {
    return i2c1.status().read().tx_em().bit_is_set();
}
/// Check if TX FIFO is full
fn tx_full(i2c1: &I2C1) -> bool {
    return i2c1.status().read().tx_full().bit_is_set();
}
/// Handle TX lockout
fn handle_tx_lockout(i2c1: &I2C1) {
    if i2c1.intfl0().read().tx_lockout().bit_is_set() {
        i2c1.intfl0().modify(|_, w| w.tx_lockout().set_bit());
    }
}

/// Read data from a slave.
/// The component MUST be waiting for the read. If the component does not 
/// send an ack, this function will spin indefinitely. This function blocks 
/// until all the data is received.
#[must_use]
pub fn master_read_bytes(i2c1: &I2C1, addr: u8, bytes: &mut [u8]) -> MasterI2CStatus {
    // Make sure read bit is set
    let addr = (addr << 1) | 0x1;
    let len = bytes.len() as u8;
    // Handle TX lockout
    handle_tx_lockout(i2c1);
    // Reset state
    clear_rxfifo(i2c1);
    i2c1.intfl0().modify(|_, w| w
        .addr_ack().set_bit()
        .addr_nack_err().set_bit()
        .done().set_bit()
    );
    while i2c1.intfl0().read().addr_ack().bit_is_set() ||
          i2c1.intfl0().read().addr_nack_err().bit_is_set() ||
          i2c1.intfl0().read().done().bit_is_set() { }
    // Write number of expected bytes
    i2c1.rxctrl1().modify(|_, w| unsafe { w.cnt().bits(len) });
    // Write the address byte of the slave
    i2c1.fifo().modify(|_, w| unsafe { w.data().bits(addr) });
    // Write the start bit
    i2c1.mstctrl().modify(|_, w| w.start().set_bit() );
    // // Wait until we either receive an ACK or NACK
    // while i2c1.intfl0().read().addr_ack().bit_is_clear() ||
    //       i2c1.intfl0().read().addr_nack_err().bit_is_clear() { }
    // // If we received a NACK, we should stop
    // if i2c1.intfl0().read().addr_nack_err().bit_is_set() {
    //     i2c1.intfl0().modify(|_, w| w.addr_nack_err().set_bit());
    //     i2c1.mstctrl().modify(|_, w| w.stop().set_bit());
    //     return MasterI2CStatus::Nack;
    // }
    // i2c1.intfl0().modify(|_, w| w.addr_ack().set_bit());
    // Read in bytes
    for byte in bytes.iter_mut() {
        while rx_em(i2c1) {
            // Terminate early if NACK is received
            if i2c1.intfl0().read().addr_nack_err().bit_is_set() {
                i2c1.mstctrl().modify(|_, w| w.stop().set_bit());
                i2c1.intfl0().modify(|_, w| w.addr_nack_err().set_bit());
                clear_rxfifo(i2c1);
                return MasterI2CStatus::Nack;
            }
        }
        *byte = i2c1.fifo().read().data().bits();
    }
    // Set stop bit
    i2c1.mstctrl().modify(|_, w| w.stop().set_bit());
    // Wait until transaction is done
    while i2c1.intfl0().read().done().bit_is_clear() { }
    i2c1.intfl0().modify(|_, w| w.done().set_bit());
    return MasterI2CStatus::Success;
}

/// Write data to a slave.
/// The slave must be waiting for the write. If the slave is not waiting and 
/// this function is called, this will spin indefinitely.
#[must_use]
pub fn master_write_bytes(i2c1: &I2C1, addr: u8, bytes: &[u8]) -> MasterI2CStatus {
    // Make sure read bit is not set
    let addr = (addr << 1) & 0xFE;
    // Handle TX lockout
    handle_tx_lockout(i2c1);
    // Reset state
    clear_txfifo(i2c1);
    i2c1.intfl0().modify(|_, w| w
        .addr_ack().set_bit()
        .addr_nack_err().set_bit()
        .data_err().set_bit()
        .done().set_bit()
    );
    while i2c1.intfl0().read().addr_ack().bit_is_set() ||
          i2c1.intfl0().read().addr_nack_err().bit_is_set() ||
          i2c1.intfl0().read().data_err().bit_is_set() ||
          i2c1.intfl0().read().done().bit_is_set() { }
    // Write the address byte of the slave
    i2c1.fifo().modify(|_, w| unsafe { w.data().bits(addr) });
    // Set start bit
    i2c1.mstctrl().modify(|_, w| w.start().set_bit());
    // Write bytes
    for byte in bytes {
        while tx_full(i2c1) {
            // Terminate early if NACK is received
            if i2c1.intfl0().read().addr_nack_err().bit_is_set() ||
                i2c1.intfl0().read().data_err().bit_is_set() {
                i2c1.mstctrl().modify(|_, w| w.stop().set_bit());
                i2c1.intfl0().modify(|_, w| w.data_err().set_bit());
                i2c1.intfl0().modify(|_, w| w.addr_nack_err().set_bit());
                clear_txfifo(i2c1);
                return MasterI2CStatus::Nack;
            }
        }
        i2c1.fifo().modify(|_, w| unsafe { w.data().bits(*byte) });
    }
    // Set stop bit
    i2c1.mstctrl().modify(|_, w| w.stop().set_bit());
    // Wait until transaction is done
    while i2c1.intfl0().read().done().bit_is_clear() { }
    i2c1.intfl0().modify(|_, w| w.done().set_bit());
    // Abort if NACK is received
    if i2c1.intfl0().read().addr_nack_err().bit_is_set() ||
        i2c1.intfl0().read().data_err().bit_is_set() {
        i2c1.mstctrl().modify(|_, w| w.stop().set_bit());
        i2c1.intfl0().modify(|_, w| w.data_err().set_bit());
        i2c1.intfl0().modify(|_, w| w.addr_nack_err().set_bit());
        clear_txfifo(i2c1);
        return MasterI2CStatus::Nack;
    }
    return MasterI2CStatus::Success;
}

/// Read data from master.
/// This function will block until the master sends all the data.
pub fn slave_read_bytes(i2c1: &I2C1, bytes: &mut [u8]) {
    // Wait until addressed by master
    while !i2c1.intfl0().read().rd_addr_match().bit_is_set() { }
    i2c1.intfl0().modify(|_, w| w.rd_addr_match().clear_bit());
    // Reset state
    handle_tx_lockout(i2c1);
    i2c1.intfl0().modify(|_, w| w.done().set_bit());
    while i2c1.intfl0().read().done().bit_is_set() { }
    // Read byte from FIFO
    for byte in bytes.iter_mut() {
        while rx_em(i2c1) { }
        *byte = i2c1.fifo().read().data().bits();
    }
    // Wait until transaction is done
    while i2c1.intfl0().read().done().bit_is_clear() { }
    i2c1.intfl0().modify(|_, w| w.done().set_bit());
}

/// Write data to master (when prompted).
/// This function will block until the master reads all the data.
pub fn slave_write_bytes(i2c1: &I2C1, bytes: &[u8]) {
    // Wait until addressed by master
    while !i2c1.intfl0().read().wr_addr_match().bit_is_set() { }
    i2c1.intfl0().modify(|_, w| w.wr_addr_match().clear_bit());
    // Reset state
    handle_tx_lockout(i2c1);
    clear_txfifo(i2c1);
    i2c1.intfl0().modify(|_, w| w.done().set_bit());
    while i2c1.intfl0().read().done().bit_is_set() { }
    // Write bytes to FIFO
    for byte in bytes {
        while tx_full(i2c1) { }
        i2c1.fifo().modify(|_, w| unsafe {
            w.data().bits(*byte)
        });
    }
    // Wait until transaction is done
    while i2c1.intfl0().read().done().bit_is_clear() { }
    i2c1.intfl0().modify(|_, w| w.done().set_bit());
}
