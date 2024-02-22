// use max78000_pac::GPIO2;
// use crate::gpio2;

// pub struct Led {
//     pins: u32,
// }

// impl Led {
//     // Create a new LED instance
//     pub fn new(reg: &GPIO2, cfg: gpio2::Gpio2Config) -> Self {
//         let pins = cfg.pins.clone();
//         gpio2::config(reg, cfg);
//         Led { pins }
//     }

//     // Turn the LED on
//     pub fn on(&self, reg: &GPIO2) {
//         gpio2::set_out(reg, self.pins);
//     }

//     // Turn the LED off
//     pub fn off(&self, reg: &GPIO2) {
//         gpio2::clr_out(reg, self.pins);
//     }

//     // Toggle the LED
//     pub fn toggle(&self, reg: &GPIO2) {
//         gpio2::toggle_out(reg, self.pins);
//     }
// }