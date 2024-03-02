pub use max78000_pac::I2C1;

pub enum MasterI2CStatus {
    Success,
    Nack,
    TxLockout,
    ArbitrationLost,
    InvalidState
}

pub enum SlaveI2CStatus {
    Success,
    Nack,
    TxLockout,
    InvalidState
}

/// Initialize I2C1
pub fn master_config(i2c1: &I2C1) {
    // Recover func call & check return
    // Disable I2C peripheral
    // i2c1.ctrl().modify(|_, w| w.en().clear_bit());
    // while i2c1.ctrl().read().en().bit_is_set() { }
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
    // Set thresholds
    // set_tx_threshold(&i2c1, 2);
    // set_rx_threshold(&i2c1, 6);
    // How do we want to do error return?
}

/// Configure I2C slave for just-in-time configuration
/// Safety: I2C should be disabled before calling this function
pub fn slave_config(i2c1: &I2C1, slave_addr: u8) {
    let addr: u8 = slave_addr & 0x7F;
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
    let mut ticksTotal = 0u16;
    let mut hiClks = 0u16;
    let mut lowClks = 0u16;

    ticksTotal = (50000000 / hz) as u16;
    hiClks = (ticksTotal >> 1) - 1;
    lowClks = (ticksTotal >> 1) - 1;

    i2c1.clklo().modify( unsafe{|_,w| w.lo().bits(lowClks)});
    i2c1.clkhi().modify( unsafe{|_,w| w.hi().bits(hiClks)});
}

fn i2c_recover(i2c1: &I2C1, retires: u32){

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

/// Check if TX lockout is set
fn is_tx_lockout(i2c1: &I2C1) -> bool {
    return i2c1.intfl0().read().tx_lockout().bit_is_set();
}

/// Handle TX lockout
fn handle_tx_lockout(i2c1: &I2C1) {
    if is_tx_lockout(i2c1) {
        i2c1.intfl0().modify(|_, w| w.tx_lockout().set_bit());
    }
}

/// Read data from a slave
pub fn master_read_bytes(i2c1: &I2C1, addr: u8, bytes: &mut [u8]) -> MasterI2CStatus {
    // Make sure read bit is set
    let addr = (addr << 1) | 0x1;
    let len = bytes.len() as u8;
    // The rx fifo should be empty when we call this function
    if !rx_em(i2c1) {
        return MasterI2CStatus::InvalidState;
    }
    // Handle TX lockout
    handle_tx_lockout(i2c1);
    // Write number of expected bytes
    i2c1.rxctrl1().modify(|_, w| unsafe { w.cnt().bits(len) });
    // Write the address byte of the slave
    i2c1.fifo().modify(|_, w| unsafe { w.data().bits(addr) });
    // Write the start bit
    i2c1.mstctrl().modify(|_, w| w.start().set_bit() );

//    // Wait until we either receive an ACK or NACK
//    while
//        i2c1.intfl0().read().addr_ack().bit_is_clear() ||
//        i2c1.intfl0().read().addr_nack_err().bit_is_clear() { }
//    // If we received a NACK, we should stop
//    if i2c1.intfl0().read().addr_nack_err().bit_is_set() {
//        i2c1.mstctrl().modify(|_, w| w.stop().set_bit());
//        return MasterI2CStatus::Nack;
//    }

    // Read in bytes
    for byte in bytes.iter_mut() {
        while rx_em(i2c1) { }
        *byte = i2c1.fifo().read().data().bits();
    }
    // Handle transaction done interrupt flag
    if i2c1.intfl0().read().done().bit_is_set() {
        i2c1.intfl0().modify(|_, w| w.done().set_bit());
        i2c1.mstctrl().modify(|_, w| unsafe { w.stop().bit(true)});
    } else {
        return MasterI2CStatus::InvalidState;
    }
    return MasterI2CStatus::Success;
}


pub fn master_write_bytes(i2c1: &I2C1, addr: u8, data: &[u8]) {
    let addr = (addr << 1) & 0xFE;
    let mut written = 0usize;
    let mut started = false;

    while written < data.len() + 1 {
       
        while !tx_em(i2c1) {}

        if !started {
            i2c1.fifo().modify(|_, w| unsafe { w.data().bits(addr) });
            written += 1;
        }
        
        while !tx_full(i2c1) && written < data.len() + 1 {
            i2c1.fifo().modify(|_, w| unsafe { w.data().bits(data[written - 1]) });
            written += 1;
        }

        // set master control start to 1
        if !started {
            i2c1.mstctrl().modify(|_, w| unsafe { w.start().bit(true)});
            started = true;
        }

    }
    i2c1.mstctrl().modify(|_, w| unsafe { w.stop().bit(true)});
}

/// Read data from master
pub fn slave_read_bytes(i2c1: &I2C1, bytes: &mut [u8]) -> SlaveI2CStatus {
    // Wait until addressed by master
    while !i2c1.intfl0().read().rd_addr_match().bit_is_set() { }
    

    i2c1.intfl0().modify(|_, w| w.addr_match().clear_bit());
    // Read byte from FIFO
    for byte in bytes.iter_mut() {
        while rx_em(i2c1) { }
        *byte = i2c1.fifo().read().data().bits();
    }
    // Check for invalid state
    if !rx_em(i2c1) ||
        i2c1.intfl1().read().rx_ov().bit_is_set() ||
        !i2c1.intfl0().read().done().bit_is_set()
    {
        clear_rxfifo(i2c1);
        return SlaveI2CStatus::InvalidState;
    }
    i2c1.intfl0().modify(|_, w| w.done().set_bit());
    return SlaveI2CStatus::Success;
}

/// Write data to master (when prompted)
pub fn slave_write_bytes(i2c1: &I2C1, bytes: &[u8]) -> SlaveI2CStatus {
    // Wait until addressed by master
    while !i2c1.intfl0().read().wr_addr_match().bit_is_set() { }
    // Ensure master requested a write

    i2c1.intfl0().modify(|_, w| w.addr_match().clear_bit());
    // There shouldn't be anything in the TX FIFO
    if !tx_em(i2c1) {
        return SlaveI2CStatus::InvalidState;
    }
    // Handle TX lockout
    handle_tx_lockout(i2c1);
    // Write bytes to FIFO
    for byte in bytes {
        while tx_full(i2c1) { }
        i2c1.fifo().modify(|_, w| unsafe { w.data().bits(*byte) });
    }
    // Check for invalid state
    if !i2c1.intfl0().read().done().bit_is_set() {
        return SlaveI2CStatus::InvalidState;
    }
    i2c1.intfl0().modify(|_, w| w.done().set_bit());
    return SlaveI2CStatus::Success;
}
