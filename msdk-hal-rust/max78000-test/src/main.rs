#![no_std]
#![no_main]

use panic_halt as _;
// use panic_semihosting as _;
use cortex_m_rt::entry;
use max78000_pac::Peripherals;

// This should go in gpio.rs
pub enum MxcGpioFunc {
    In = 0,
    Out = 1,
    Alt1 = 2,
    Alt2 = 3,
}
pub enum MxcGpioPad {
    None = 0,
    PullUp = 1,
    PullDown = 2,
    WeakPullUp = 3,
    WeakPullDown = 4,
}
pub enum MxcGpioVSSel {
    Vddio = 0,
    Vddioh = 1,
}
/// Configure alternate function for a GPIO pin
/// Safety: Caller must ensure that the function and mask are valid
/// Source: msdk/Libraries/PeriphDrivers/Source/GPIO/gpio_reva.c
unsafe fn mxc_gpio0_reva_set_af(p: &Peripherals, func: MxcGpioFunc, mask: u32) {
    p.GPIO0.inen().modify(|r, w|
        w.bits(r.bits() | mask)
    );
    // Switch to I/O mode first
    p.GPIO0.en0_set().write(|w|
        w.bits(mask)
    );
    match func {
        // en3 is not available on the MAX78000
        MxcGpioFunc::In => {
            p.GPIO0.outen_clr().write(|w|
                w.bits(mask)
            );
            p.GPIO0.en0_set().write(|w|
                w.bits(mask)
            );
            p.GPIO0.en1_clr().write(|w|
                w.bits(mask)
            );
            p.GPIO0.en2_clr().write(|w|
                w.bits(mask)
            );
        }
        MxcGpioFunc::Out => {
            p.GPIO0.outen_set().write(|w|
                w.bits(mask)
            );
            p.GPIO0.en0_set().write(|w|
                w.bits(mask)
            );
            p.GPIO0.en1_clr().write(|w|
                w.bits(mask)
            );
            p.GPIO0.en2_clr().write(|w|
                w.bits(mask)
            );
        }
        MxcGpioFunc::Alt1 => {
            p.GPIO0.en0_clr().write(|w|
                w.bits(mask)
            );
            p.GPIO0.en1_clr().write(|w|
                w.bits(mask)
            );
            p.GPIO0.en2_clr().write(|w|
                w.bits(mask)
            );
        }
        MxcGpioFunc::Alt2 => {
            p.GPIO0.en0_clr().write(|w|
                w.bits(mask)
            );
            p.GPIO0.en1_set().write(|w|
                w.bits(mask)
            );
            p.GPIO0.en2_clr().write(|w|
                w.bits(mask)
            );
        }
    }
}

/// Configure pad for a GPIO pin
/// Safety: Caller must ensure that the pad and mask are valid
/// /// Source: msdk/Libraries/PeriphDrivers/Source/GPIO/gpio_ai85.c
unsafe fn mxc_gpio0_set_pad(p: &Peripherals, pad: MxcGpioPad, mask: u32) {
    match pad {
        MxcGpioPad::None => {
            p.GPIO0.padctrl0().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
            p.GPIO0.padctrl1().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
        }
        MxcGpioPad::PullUp => {
            p.GPIO0.padctrl0().modify(|r, w|
                w.bits(r.bits() | mask)
            );
            p.GPIO0.padctrl1().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
            p.GPIO0.ps().modify(|r, w|
                w.bits(r.bits() | mask)
            );
        }
        MxcGpioPad::PullDown => {
            p.GPIO0.padctrl0().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
            p.GPIO0.padctrl1().modify(|r, w|
                w.bits(r.bits() | mask)
            );
            p.GPIO0.ps().modify(|r, w|
                w.bits(r.bits() | (mask))
            );
        }
        MxcGpioPad::WeakPullUp => {
            p.GPIO0.padctrl0().modify(|r, w|
                w.bits(r.bits() | (mask))
            );
            p.GPIO0.padctrl1().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
            p.GPIO0.ps().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
        }
        MxcGpioPad::WeakPullDown => {
            p.GPIO0.padctrl0().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
            p.GPIO0.padctrl1().modify(|r, w|
                w.bits(r.bits() | (mask))
            );
            p.GPIO0.ps().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
        }
    }
}

/// Configure voltage supply select for a GPIO pin
/// Safety: Caller must ensure that the vssel and mask are valid
/// Source: msdk/Libraries/PeriphDrivers/Source/GPIO/gpio_reva.c
unsafe fn mxc_gpio0_reva_set_vssel(p: &Peripherals, vssel: MxcGpioVSSel, mask: u32) {
    match vssel {
        MxcGpioVSSel::Vddio => {
            p.GPIO0.vssel().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
        }
        MxcGpioVSSel::Vddioh => {
            p.GPIO0.vssel().modify(|r, w|
                w.bits(r.bits() | mask)
            );
        }
    }
}
// This should go in gpio.rs

#[entry]
fn main() -> ! {
    let mut count: u32 = 0;
    // Safety: We only steal once and we have exclusive access to the peripherals
    let peripherals: Peripherals = unsafe { Peripherals::steal() };
    let mut array: [u32; 10] = [
        0xdeadbeef,
        0x00000001,
        0xfeedface,
        0x00000002,
        0xfeedface,
        0x00000003,
        0xfeedface,
        0x00000004,
        0xfeedface,
        0xdeadc0de
    ];

    // Setup system clock
    system_clock_ipo_init(&peripherals);

    // Initialize GPIO0
    let gpio_status = mxc_gpio0_init(&peripherals);
    if gpio_status != 0 {
        array[9] = 0x99999999;
    }

    // Initialize the UART for 115200 baud, 8N1
    let uart_status = uart_init(&peripherals);
    if uart_status != 0 {
        array[9] = 0xdddddddd;
    }

    uart_send(&peripherals.UART, "UART initialized!\r\n");

    // Initialize TMR0 as continuous 32-bit system tick timer
    tmr0_init(&peripherals);

    // Success
    uart_send(&peripherals.UART, "Initialization success!\r\n");

    loop {
        let tmr_cnt = tmr0_get_tick_count(&peripherals);
        while tmr0_get_tick_count(&peripherals) < tmr_cnt + 50_000_000 { }
        uart_send(&peripherals.UART, "Hello, world!\r\n");
        count += 1;
        continue;
    }
}

/// Initialize TMR0 as continuous 32-bit system tick timer
fn tmr0_init(p: &Peripherals) {
    uart_send(&p.UART, "Initializing TMR0...\r\n");
    // Standard peripheral setup
    mxc_tmr0_shutdown(p);
    // mxc_tmr0_reset(p);
    mxc_tmr0_enable_clock(p);
    mxc_tmr0_config_gpio(p);
    // Configure TMR0
    tmr0_config(p);
}

/// Disable the TMR0 peripheral clock
fn mxc_tmr0_shutdown(p: &Peripherals) {
    p.GCR.pclkdis0().modify(|_, w| w.tmr0().set_bit());
    while p.GCR.pclkdis0().read().tmr0().bit_is_clear() { }
    uart_send(&p.UART, "TMR0 shutdown complete!\r\n");
}

/// Reset the TMR0 peripheral
fn mxc_tmr0_reset(p: &Peripherals) {
    p.GCR.rst0().modify(|_, w| w.tmr0().set_bit());
    while p.GCR.rst0().read().tmr0().bit_is_clear() { }
    uart_send(&p.UART, "TMR0 reset complete!\r\n");
}

/// Enable the TMR0 peripheral clock
fn mxc_tmr0_enable_clock(p: &Peripherals) {
    p.GCR.pclkdis0().modify(|_, w| w.tmr0().clear_bit());
    while p.GCR.pclkdis0().read().tmr0().bit_is_set() { }
    uart_send(&p.UART, "TMR0 clock enabled!\r\n");
}

/// Configure pins for TMR0
/// - P0.2 (AF1): TMR0_0
/// Configure pad (none)
/// Configure voltage select (VDDIO)
/// Source: MAX78000FTHR Schematic
/// Source: msdk/Libraries/PeriphDrivers/Source/SYS/pins_ai85.c
fn mxc_tmr0_config_gpio(p: &Peripherals) {
    mxc_gpio0_init(p);
    // Source: gpio_cfg_tmr0
    let mask: u32 = 1 << 2;
    let func: MxcGpioFunc = MxcGpioFunc::Alt1;
    let pad: MxcGpioPad = MxcGpioPad::None;
    let vssel: MxcGpioVSSel = MxcGpioVSSel::Vddio;
    // Safety: Configuration options are valid
    unsafe {
        mxc_gpio0_reva_set_af(p, func, mask);
        mxc_gpio0_set_pad(p, pad, mask);
        mxc_gpio0_reva_set_vssel(p, vssel, mask);
    };
    uart_send(&p.UART, "TMR0 GPIO configured!\r\n");
}

/// Disable the TMR0 peripheral
fn mxc_tmr0_disable(p: &Peripherals) {
    uart_send(&p.UART, "Disabling TMR0...\r\n");
    p.TMR.ctrl0().modify(|_, w| w
        .en_a().clear_bit()
        .en_b().clear_bit()
    );
    while p.TMR.ctrl1().read().clken_a().bit_is_set() { } // For some reason this is CTRL1
    while p.TMR.ctrl1().read().clken_b().bit_is_set() { }
    uart_send(&p.UART, "TMR0 disabled!\r\n");
}

/// Enable the TMR0 peripheral
fn mxc_tmr0_enable(p: &Peripherals) {
    uart_send(&p.UART, "Enabling TMR0...\r\n");
    p.TMR.ctrl0().modify(|_, w| w
        .en_a().set_bit()
        // .en_b().set_bit()
    );
    while p.TMR.ctrl0().read().clken_a().bit_is_clear() { } // For some reason this is CTRL0
    // while p.TMR.ctrl0().clken_b().bit_is_clear() { }
    uart_send(&p.UART, "TMR0 enabled!\r\n");
}

/// Configure TMR0:
/// - 50MHz peripheral clock (no prescaler)
/// - Continuous mode
/// - 32-bit cascade mode
/// - Range: 0x1 to 0xFFFFFFFF
fn tmr0_config(p: &Peripherals) {
    uart_send(&p.UART, "Configuring TMR0...\r\n");
    mxc_tmr0_disable(p);
    // Set the TMR0 clock source to the peripheral clock
    // Safety: The clksel_a field is 2 bits wide, which fits the value 0b00
    p.TMR.ctrl1().modify(|_, w| unsafe {
        w.clksel_a().bits(0b00)
    });
    // Set 32-bit cascade mode
    p.TMR.ctrl1().modify(|_, w| w .cascade().set_bit());
    p.TMR.ctrl0().modify(|_, w| w
        .pol_a().clear_bit()        // Set polarity to active high
        .clkdiv_a().div_by_1()      // Set prescaler to divide by 1
        .mode_a().continuous()      // Set continuous mode
    );
    // Set initial count to 0x1
    p.TMR.cnt().write(|w| unsafe { w.bits(0x1) });
    // Set the compare value to 0xFFFFFFFF
    p.TMR.cmp().write(|w| unsafe { w.bits(0xFFFFFFFF) });
    // Enable timer clock source
    p.TMR.ctrl0().modify(|_, w| w.clken_a().set_bit());
    while p.TMR.ctrl1().read().clkrdy_a().bit_is_clear() { }
    // Enable timer
    mxc_tmr0_enable(p);
    uart_send(&p.UART, "TMR0 configured!\r\n");
}

fn tmr0_get_tick_count(p: &Peripherals) -> u32 {
    return p.TMR.cnt().read().bits();
}

fn system_clock_ipo_init(p: &Peripherals) {
    // Enable the Internal Primary Oscillator (IPO)
    p.GCR.clkctrl().modify(|_, w| w.ipo_en().en());
    // Wait for IPO to be ready
    while p.GCR.clkctrl().read().ipo_rdy().bit_is_clear() { }
    // Set the system clock to IPO and the divisor to 1
    p.GCR.clkctrl().modify(|_, w| w
        .sysclk_sel().ipo()
        .sysclk_div().div1()
    );
    // Wait for the system clock to be ready
    while p.GCR.clkctrl().read().sysclk_rdy().bit_is_clear() { }
}

fn mxc_gpio0_init(p: &Peripherals) -> i32 {
    // Enable the GPIO0 peripheral clock
    p.GCR.pclkdis0().modify(|_, w| w.gpio0().clear_bit());
    // TODO: Configure GPIO callback initialization (4x32 null function pointers)
    // See msdk/Libraries/PeriphDrivers/Source/GPIO/gpio_common.c, MXC_GPIO_Common_Init
    return 0;
}

fn mxc_gpio0_uart0_init(peripherals: &Peripherals) {
    // TODO: Should shutdown UART0 first

    // Configure GPIO pins P0_0 and P0_1 as UART0 RX and TX (AF1)
    // Source: MAX78000FTHR data sheet
    // Source: msdk/Libraries/PeriphDrivers/Source/SYS/pins_ai85.c
    // Source: msdk/Libraries/PeriphDrivers/Source/GPIO/gpio_ai85.c
    // STEP 1: Configure alternate function 1 for P0_0 and P0_1
    let mask = (1 << 0) | (1 << 1);
    // Safety: See MXC_GPIO_RevA_SetAF() in msdk/Libraries/PeriphDrivers/Source/GPIO/gpio_reva.c
    peripherals.GPIO0.inen().modify(|r, w| unsafe {
        w.bits(r.bits() | mask)
    });
    peripherals.GPIO0.en0_set().write(|w| unsafe {
        w.bits(mask)
    });
    // en3 is not available on the MAX78000
    // peripherals.GPIO0.en3_clr().write(|w| unsafe {
    //     w.bits(mask)
    // });
    peripherals.GPIO0.en2_clr().write(|w| unsafe {
        w.bits(mask)
    });
    peripherals.GPIO0.en1_clr().write(|w| unsafe {
        w.bits(mask)
    });
    peripherals.GPIO0.en0_clr().write(|w| unsafe {
        w.bits(mask)
    });
    // STEP 2: Configure pad (none)
    // Safety: idk man
    peripherals.GPIO0.padctrl0().modify(|r, w| unsafe {
        w.bits(r.gpio_padctrl0().bits() & !(mask))
    });
    peripherals.GPIO0.padctrl1().modify(|r, w| unsafe {
        w.bits(r.gpio_padctrl1().bits() & !(mask))
    });
    // STEP 3: Configure the voltage select
    // TODO: VDDIO or VDDIOH? By default it's VDDIO
}

fn uart_init(peripherals: &Peripherals) -> u32 {
    // Configure GPIO pins P0_0 and P0_1 as UART0 RX and TX (AF1)
    mxc_gpio0_uart0_init(peripherals);
    // Enable UART0 peripheral clock
    peripherals.GCR.pclkdis0().modify(|_, w| w.uart0().clear_bit());
    // Set RX threshold to 1 byte
    peripherals.UART.ctrl().write(|w| unsafe {
        w.rx_thd_val().bits(1)
    });
    // Set the UART clock and configure 8N1
    peripherals.UART.ctrl().write(|w| w
        .ucagm().set_bit()
        .bclksrc().peripheral_clock()
        .bclken().set_bit()
        .stopbits().clear_bit()
        .char_size()._8bits()
        .par_en().clear_bit()
    );
    // Set the baud rate to 115200
    // Assuming that the Peripheral Clock is 50MHz (System Clock 100MHz / 2)
    // 50MHz / 115200 = 434.0277
    // Safety: The clkdiv field is 19 bits wide, which fits the divisor 434
    peripherals.UART.clkdiv().write(|w| unsafe { w.clkdiv().bits(434) });
    // Wait until baud clock is ready
    while !peripherals.UART.ctrl().read().bclkrdy().bit_is_set() {}
    return 0;
}

fn uart_send_byte(uart: &max78000_pac::UART, byte: u8) {
    // Wait until the TX FIFO is not full
    while uart.status().read().tx_full().bit_is_set() {}
    // Transmit the byte to the UART FIFO register
    // Safety: The data field is 8 bits wide, which fits the byte
    uart.fifo().write(|w| unsafe { w.data().bits(byte) });
}

fn uart_send(uart: &max78000_pac::UART, message: &str) {
    for byte in message.bytes() {
        uart_send_byte(uart, byte);
    }
}