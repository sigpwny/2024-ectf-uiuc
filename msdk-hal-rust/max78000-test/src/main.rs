#![no_std]
#![no_main]

use panic_halt as _;
// use panic_semihosting as _;
use cortex_m_rt::entry;
use max78000_pac::Peripherals;
// use max78000_pac::UART;

#[entry]
fn main() -> ! {
    let mut int32: i32 = 0;
    // Safety: We only steal once and we have exclusive access to the peripherals
    let peripherals: Peripherals = unsafe { Peripherals::steal() };
    // let flc: FLC = peripherals.FLC;
    // let status = uart.status().read().bits();
    let uart_ctrl_reader = peripherals.UART.ctrl().read();
    let uart_bclksrc: u32 = uart_ctrl_reader.bclksrc().bits().into();
    // flc.addr().reset();
    // let flc_clkdiv: u32 = flc.clkdiv().read().clkdiv().bits().into();
    let mut array = [
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
    array[2] = uart_bclksrc;
    // array[4] = flc_clkdiv;

    // TODO: Setup peripheral clock (GCR.pclkdiv)

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

    // Check that bclksrc is set to peripheral_clock (do while loop)
    // let mut uart_bclksrc_new: u32 = uart_ctrl_reader.bclksrc().bits().into();
    // let mut is_peripheral_clock: bool = uart_ctrl_reader.bclksrc().is_clk3();
    // let mut i: u32 = 0;
    // array[4] = uart_bclksrc_new;
    // while !is_peripheral_clock {
    //     uart_bclksrc_new = uart_ctrl_reader.bclksrc().bits().into();
    //     is_peripheral_clock = uart_ctrl_reader.bclksrc().is_clk3();
    //     array[6] = uart_bclksrc_new;
    //     i += 1;
    //     array[8] = i;
    //     continue;
    // }
    // Success
    array[9] = 0xeeeeeeee;


    loop {
        // let new_flc_addr: u32 = flc.addr().read().addr().bits();
        // array[4] = new_flc_addr;
        if array[9] == 0xaaaaaaaa {
            array[9] = 0xbbbbbbbb;
            let message = "Hello, world!\n";
            for byte in message.bytes() {
                uart_send_byte(&peripherals.UART, byte);
            }
        }
        int32 += 1;
        continue;
    }
}

fn mxc_gpio0_init(peripherals: &Peripherals) -> i32 {
    // Enable the GPIO0 peripheral clock
    peripherals.GCR.pclkdis0().modify(|_, w| w.gpio0().clear_bit());
    // TODO: Configure GPIO callback initialization (4x32 null function pointers)
    // See msdk/Libraries/PeriphDrivers/Source/GPIO/gpio_common.c, MXC_GPIO_Common_Init
    return 0;
}

fn mxc_gpio0_uart0_init(peripherals: &Peripherals) {
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