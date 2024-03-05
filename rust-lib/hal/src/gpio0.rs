pub use max78000_pac::GPIO0;

pub enum Gpio0Pin {
    Pin0 = 1 << 0,
    Pin1 = 1 << 1,
    Pin2 = 1 << 2,
    Pin3 = 1 << 3,
    Pin4 = 1 << 4,
    Pin5 = 1 << 5,
    Pin6 = 1 << 6,
    Pin7 = 1 << 7,
    Pin8 = 1 << 8,
    Pin9 = 1 << 9,
    Pin10 = 1 << 10,
    Pin11 = 1 << 11,
    Pin12 = 1 << 12,
    Pin13 = 1 << 13,
    Pin14 = 1 << 14,
    Pin15 = 1 << 15,
    Pin16 = 1 << 16,
    Pin17 = 1 << 17,
    Pin18 = 1 << 18,
    Pin19 = 1 << 19,
    Pin20 = 1 << 20,
    Pin21 = 1 << 21,
    Pin22 = 1 << 22,
    Pin23 = 1 << 23,
    Pin24 = 1 << 24,
    Pin25 = 1 << 25,
    Pin26 = 1 << 26,
    Pin27 = 1 << 27,
    Pin28 = 1 << 28,
    Pin29 = 1 << 29,
    Pin30 = 1 << 30,
}
pub enum Gpio0Func {
    In = 0,
    Out = 1,
    Alt1 = 2,
    Alt2 = 3,
}
pub enum Gpio0Pad {
    None = 0,
    PullUp = 1,
    PullDown = 2,
    WeakPullUp = 3,
    WeakPullDown = 4,
}
pub enum Gpio0VSSel {
    Vddio = 0,
    Vddioh = 1,
}
pub struct Gpio0Config {
    pub pins: u32,
    pub func: Gpio0Func,
    pub pad: Gpio0Pad,
    pub vssel: Gpio0VSSel,
}

/// Pin config for TMR0
pub const GPIO0_CFG_TMR0: Gpio0Config = Gpio0Config {
    pins: Gpio0Pin::Pin2 as u32,
    func: Gpio0Func::Alt1,
    pad: Gpio0Pad::None,
    vssel: Gpio0VSSel::Vddio,
};

/// Pin config for TMR1
pub const GPIO0_CFG_TMR1: Gpio0Config = Gpio0Config {
    pins: Gpio0Pin::Pin14 as u32,
    func: Gpio0Func::Alt1,
    pad: Gpio0Pad::None,
    vssel: Gpio0VSSel::Vddio,
};

/// Pin config for TMR2
pub const GPIO0_CFG_TMR2: Gpio0Config = Gpio0Config {
    pins: Gpio0Pin::Pin26 as u32,
    func: Gpio0Func::Alt1,
    pad: Gpio0Pad::None,
    vssel: Gpio0VSSel::Vddio,
};

/// Pin config for UART0 TX and RX
pub const GPIO0_CFG_UART0: Gpio0Config = Gpio0Config {
    pins: (Gpio0Pin::Pin0 as u32) | (Gpio0Pin::Pin1 as u32),
    func: Gpio0Func::Alt1,
    pad: Gpio0Pad::None,
    vssel: Gpio0VSSel::Vddio,
};

/// Pin config for I2C1 SCL and SDA
pub const GPIO0_CFG_I2C1: Gpio0Config = Gpio0Config {
    pins: (Gpio0Pin::Pin16 as u32) | (Gpio0Pin::Pin17 as u32),
    func: Gpio0Func::Alt1,
    pad: Gpio0Pad::None,
    vssel: Gpio0VSSel::Vddio,
};

/// Configure a GPIO0 peripheral
pub fn config(gpio0: &GPIO0, config: Gpio0Config) {
    // Safety: Only valid configs are allowed
    unsafe {
        mxc_gpio0_reva_set_af(gpio0, config.func, config.pins);
        mxc_gpio0_set_pad(gpio0, config.pad, config.pins);
        mxc_gpio0_reva_set_vssel(gpio0, config.vssel, config.pins);
    }
}

/// Configure alternate function for a GPIO pin
/// Safety: Caller must ensure that the function and mask are valid
/// Source: msdk/Libraries/PeriphDrivers/Source/GPIO/gpio_reva.c
unsafe fn mxc_gpio0_reva_set_af(gpio0: &GPIO0, func: Gpio0Func, mask: u32) {
    gpio0.inen().modify(|r, w|
        w.bits(r.bits() | mask)
    );
    // Switch to I/O mode first
    gpio0.en0_set().write(|w|
        w.bits(mask)
    );
    match func {
        // en3 is not available on the MAX78000
        Gpio0Func::In => {
            gpio0.outen_clr().write(|w|
                w.bits(mask)
            );
            gpio0.en0_set().write(|w|
                w.bits(mask)
            );
            gpio0.en1_clr().write(|w|
                w.bits(mask)
            );
            gpio0.en2_clr().write(|w|
                w.bits(mask)
            );
        }
        Gpio0Func::Out => {
            gpio0.outen_set().write(|w|
                w.bits(mask)
            );
            gpio0.en0_set().write(|w|
                w.bits(mask)
            );
            gpio0.en1_clr().write(|w|
                w.bits(mask)
            );
            gpio0.en2_clr().write(|w|
                w.bits(mask)
            );
        }
        Gpio0Func::Alt1 => {
            gpio0.en0_clr().write(|w|
                w.bits(mask)
            );
            gpio0.en1_clr().write(|w|
                w.bits(mask)
            );
            gpio0.en2_clr().write(|w|
                w.bits(mask)
            );
        }
        Gpio0Func::Alt2 => {
            gpio0.en0_clr().write(|w|
                w.bits(mask)
            );
            gpio0.en1_set().write(|w|
                w.bits(mask)
            );
            gpio0.en2_clr().write(|w|
                w.bits(mask)
            );
        }
    }
}

/// Configure pad for a GPIO pin
/// Safety: Caller must ensure that the pad and mask are valid
/// /// Source: msdk/Libraries/PeriphDrivers/Source/GPIO/gpio_ai85.c
unsafe fn mxc_gpio0_set_pad(gpio0: &GPIO0, pad: Gpio0Pad, mask: u32) {
    match pad {
        Gpio0Pad::None => {
            gpio0.padctrl0().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
            gpio0.padctrl1().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
        }
        Gpio0Pad::PullUp => {
            gpio0.padctrl0().modify(|r, w|
                w.bits(r.bits() | mask)
            );
            gpio0.padctrl1().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
            gpio0.ps().modify(|r, w|
                w.bits(r.bits() | mask)
            );
        }
        Gpio0Pad::PullDown => {
            gpio0.padctrl0().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
            gpio0.padctrl1().modify(|r, w|
                w.bits(r.bits() | mask)
            );
            gpio0.ps().modify(|r, w|
                w.bits(r.bits() | (mask))
            );
        }
        Gpio0Pad::WeakPullUp => {
            gpio0.padctrl0().modify(|r, w|
                w.bits(r.bits() | (mask))
            );
            gpio0.padctrl1().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
            gpio0.ps().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
        }
        Gpio0Pad::WeakPullDown => {
            gpio0.padctrl0().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
            gpio0.padctrl1().modify(|r, w|
                w.bits(r.bits() | (mask))
            );
            gpio0.ps().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
        }
    }
}

/// Configure voltage supply select for a GPIO pin
/// Safety: Caller must ensure that the vssel and mask are valid
/// Source: msdk/Libraries/PeriphDrivers/Source/GPIO/gpio_reva.c
unsafe fn mxc_gpio0_reva_set_vssel(gpio0: &GPIO0, vssel: Gpio0VSSel, mask: u32) {
    match vssel {
        Gpio0VSSel::Vddio => {
            gpio0.vssel().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
        }
        Gpio0VSSel::Vddioh => {
            gpio0.vssel().modify(|r, w|
                w.bits(r.bits() | mask)
            );
        }
    }
}