pub use max78000_pac::I2C1;

/// Initialize I2C1
pub fn config(i2c1: &I2C1, master_mode: bool, slave_addr: u32) {
    // Recover func call & check return


    // i2c_recover(&i2c1, 16); TODO implement

    // set bit 0 in control register
    i2c1.ctrl().modify(|_,w| w.en().set_bit());
    // Clear FIFOs
    clear_txfifo(&i2c1);
    clear_rxfifo(&i2c1);
    // Set thresholds
    set_tx_threshold(&i2c1, 2);
    set_rx_threshold(&i2c1, 6);
    
    if !master_mode {
        // Set periph addr
        set_slave_addr(&i2c1, slave_addr);
        
    } else {
        // Set ctrl reg
        i2c1.ctrl().modify(|_,w| w.mst_mode().set_bit());
        
    }

    // Set frequency
    set_frequency(&i2c1, 100000);


    // How do we want to do error return?
    return;

}

pub fn set_slave_addr(i2c1: &I2C1, slave_addr: u32) {

    // Check addr is < MXC_F_I2C_REVA_SLAVE_ADDR

    // Set reg

}

fn set_frequency(i2c1: &I2C1, hz: u32){
    let mut ticksTotal = 0u16;
    let mut hiClks = 0u16;
    let mut lowClks = 0u16;

    ticksTotal = (50000000 / hz) as u16;
    hiClks = (ticksTotal >> 1) - 1;
    lowClks = (ticksTotal >> 1) - 1;


    i2c1.clklo().modify( unsafe{|_,w| w.lo().bits(lowClks)});
    i2c1.clkhi().modify( unsafe{|_,w| w.hi().bits(hiClks)});

    return;

}

fn i2c_recover(i2c1: &I2C1, retires: u32){

}

fn clear_txfifo(i2c1: &I2C1){
    i2c1.txctrl0().modify(|_,w| w.flush().flush());
    while i2c1.txctrl0().read().flush().is_flush() {}
}

fn clear_rxfifo(i2c1: &I2C1){
    i2c1.rxctrl0().modify(|_,w| w.flush().flush());
    while i2c1.rxctrl0().read().flush().is_flush() {}
}

fn set_tx_threshold(i2c1: &I2C1, threshold: u32){

}

fn set_rx_threshold(i2c1: &I2C1, threshold: u32){
    
}


fn rx_em(i2c1: &I2C1) -> bool {
    return i2c1.status().read().rx_em().bit_is_set();
}

fn rx_full(i2c1: &I2C1) -> bool {
    return i2c1.status().read().rx_full().bit_is_set();
}

fn tx_em(i2c1: &I2C1) -> bool {
    return i2c1.status().read().tx_em().bit_is_set();
}

fn tx_full(i2c1: &I2C1) -> bool {
    return i2c1.status().read().tx_full().bit_is_set();
}

pub fn read_rxfifo(i2c1: &I2C1, data: &mut [u8], len: u32) {
    // Block until RX FIFO is not empty
    let read = 0;
    while rx_em(i2c1) { }
    while len > read && !rx_em(i2c1) {
        
    }
}

pub fn write_txfifo(i2c1: &I2C1, data: &[u8], len: u32)  -> usize {
    //block until tx fifo is empty
    let mut written = 0;
    while !tx_em(i2c1) {}
    while !tx_full(i2c1) {
        i2c1.fifo().modify(|_, w| unsafe { w.data().bits(data[written]) });
        written += 1;
    }
    return written;
}

pub fn master_read(i2c1: &I2C1, addr: u32, data: &mut [u8], len: u32) {

}

pub fn master_write(i2c1: &I2C1, addr: u32, data: &[u8], len: usize) {

    let mut written = 0usize;
    let mut started = false;

    while written < len{
        
        while !tx_em(i2c1) {}
        
        while !tx_full(i2c1) && written < len {
            i2c1.fifo().modify(|_, w| unsafe { w.data().bits(data[written]) });
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

pub fn slave_read(i2c1: &I2C1, data: &mut [u8], len: u32) {

}

pub fn slave_write(i2c1: &I2C1, data: &[u8], len: u32) {

}
