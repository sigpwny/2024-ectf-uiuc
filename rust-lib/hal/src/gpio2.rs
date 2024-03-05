pub use max78000_pac::GPIO2;

// Too lazy to set up generics LOL

pub enum Gpio2Pin {
    Pin0 = 1 << 0,
    Pin1 = 1 << 1,
    Pin2 = 1 << 2,
    Pin3 = 1 << 3,
    Pin4 = 1 << 4,
    Pin5 = 1 << 5,
    Pin6 = 1 << 6,
    Pin7 = 1 << 7,
}
pub enum Gpio2Func {
    In = 0,
    Out = 1,
    Alt1 = 2,
    Alt2 = 3,
}
pub enum Gpio2Pad {
    None = 0,
    PullUp = 1,
    PullDown = 2,
    WeakPullUp = 3,
    WeakPullDown = 4,
}
pub enum Gpio2VSSel {
    Vddio = 0,
    Vddioh = 1,
}
pub struct Gpio2Config {
    pub pins: u32,
    pub func: Gpio2Func,
    pub pad: Gpio2Pad,
    pub vssel: Gpio2VSSel,
}

/// Pin config for LED0
pub const GPIO2_CFG_LED0: Gpio2Config = Gpio2Config {
    pins: Gpio2Pin::Pin0 as u32,
    func: Gpio2Func::Out,
    pad: Gpio2Pad::None,
    vssel: Gpio2VSSel::Vddioh,
};

/// Pin config for LED1
pub const GPIO2_CFG_LED1: Gpio2Config = Gpio2Config {
    pins: Gpio2Pin::Pin1 as u32,
    func: Gpio2Func::Out,
    pad: Gpio2Pad::None,
    vssel: Gpio2VSSel::Vddioh,
};

/// Pin config for LED2
pub const GPIO2_CFG_LED2: Gpio2Config = Gpio2Config {
    pins: Gpio2Pin::Pin2 as u32,
    func: Gpio2Func::Out,
    pad: Gpio2Pad::None,
    vssel: Gpio2VSSel::Vddioh,
};

/// Pin config for TMR4
pub const GPIO2_CFG_TMR4: Gpio2Config = Gpio2Config {
    pins: Gpio2Pin::Pin4 as u32,
    func: Gpio2Func::Alt2,
    pad: Gpio2Pad::None,
    vssel: Gpio2VSSel::Vddio,
};

/// Configure a GPIO2 peripheral
pub fn config(gpio2: &GPIO2, config: Gpio2Config) {
    // Safety: Only valid configs are allowed
    unsafe {
        mxc_gpio2_reva_set_af(gpio2, config.func, config.pins);
        mxc_gpio2_set_pad(gpio2, config.pad, config.pins);
        mxc_gpio2_reva_set_vssel(gpio2, config.vssel, config.pins);
    }
}

/// Set out for a GPIO pin
pub fn set_out(gpio2: &GPIO2, mask: u32) {
    // Safety: Only valid pins are allowed
    gpio2.out_set().write(|w| unsafe {
        w.bits(mask)
    });
    while gpio2.out().read().bits() & mask != mask {}
}

/// Clear out for a GPIO pin
pub fn clr_out(gpio2: &GPIO2, mask: u32) {
    // Safety: Only valid pins are allowed
    gpio2.out_clr().write(|w| unsafe {
        w.bits(mask)
    });
    while gpio2.out().read().bits() & mask != 0 {}
}

/// Toggle out for a GPIO pin
pub fn toggle_out(gpio2: &GPIO2, mask: u32) {
    // Safety: Only valid pins are allowed
    gpio2.out().modify(|r, w| unsafe {
        w.bits(r.bits() ^ mask)
    });
}

/// Configure alternate function for a GPIO pin
/// Safety: Caller must ensure that the function and mask are valid
/// Source: msdk/Libraries/PeriphDrivers/Source/GPIO/gpio_reva.c
unsafe fn mxc_gpio2_reva_set_af(gpio2: &GPIO2, func: Gpio2Func, mask: u32) {
    gpio2.inen().modify(|r, w|
        w.bits(r.bits() | mask)
    );
    // Switch to I/O mode first
    gpio2.en0_set().write(|w|
        w.bits(mask)
    );
    match func {
        // en3 is not available on the MAX78000
        Gpio2Func::In => {
            gpio2.outen_clr().write(|w|
                w.bits(mask)
            );
            gpio2.en0_set().write(|w|
                w.bits(mask)
            );
            gpio2.en1_clr().write(|w|
                w.bits(mask)
            );
            gpio2.en2_clr().write(|w|
                w.bits(mask)
            );
        }
        Gpio2Func::Out => {
            gpio2.outen_set().write(|w|
                w.bits(mask)
            );
            gpio2.en0_set().write(|w|
                w.bits(mask)
            );
            gpio2.en1_clr().write(|w|
                w.bits(mask)
            );
            gpio2.en2_clr().write(|w|
                w.bits(mask)
            );
        }
        Gpio2Func::Alt1 => {
            gpio2.en0_clr().write(|w|
                w.bits(mask)
            );
            gpio2.en1_clr().write(|w|
                w.bits(mask)
            );
            gpio2.en2_clr().write(|w|
                w.bits(mask)
            );
        }
        Gpio2Func::Alt2 => {
            gpio2.en0_clr().write(|w|
                w.bits(mask)
            );
            gpio2.en1_set().write(|w|
                w.bits(mask)
            );
            gpio2.en2_clr().write(|w|
                w.bits(mask)
            );
        }
    }
}

/// Configure pad for a GPIO pin
/// Safety: Caller must ensure that the pad and mask are valid
/// /// Source: msdk/Libraries/PeriphDrivers/Source/GPIO/gpio_ai85.c
unsafe fn mxc_gpio2_set_pad(gpio2: &GPIO2, pad: Gpio2Pad, mask: u32) {
    match pad {
        Gpio2Pad::None => {
            gpio2.padctrl0().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
            gpio2.padctrl1().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
        }
        Gpio2Pad::PullUp => {
            gpio2.padctrl0().modify(|r, w|
                w.bits(r.bits() | mask)
            );
            gpio2.padctrl1().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
            gpio2.ps().modify(|r, w|
                w.bits(r.bits() | mask)
            );
        }
        Gpio2Pad::PullDown => {
            gpio2.padctrl0().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
            gpio2.padctrl1().modify(|r, w|
                w.bits(r.bits() | mask)
            );
            gpio2.ps().modify(|r, w|
                w.bits(r.bits() | (mask))
            );
        }
        Gpio2Pad::WeakPullUp => {
            gpio2.padctrl0().modify(|r, w|
                w.bits(r.bits() | (mask))
            );
            gpio2.padctrl1().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
            gpio2.ps().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
        }
        Gpio2Pad::WeakPullDown => {
            gpio2.padctrl0().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
            gpio2.padctrl1().modify(|r, w|
                w.bits(r.bits() | (mask))
            );
            gpio2.ps().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
        }
    }
}

/// Configure voltage supply select for a GPIO pin
/// Safety: Caller must ensure that the vssel and mask are valid
/// Source: msdk/Libraries/PeriphDrivers/Source/GPIO/gpio_reva.c
unsafe fn mxc_gpio2_reva_set_vssel(gpio2: &GPIO2, vssel: Gpio2VSSel, mask: u32) {
    match vssel {
        Gpio2VSSel::Vddio => {
            gpio2.vssel().modify(|r, w|
                w.bits(r.bits() & !(mask))
            );
        }
        Gpio2VSSel::Vddioh => {
            gpio2.vssel().modify(|r, w|
                w.bits(r.bits() | mask)
            );
        }
    }
}