mod driverwrapper {
  #[link(name = "driverwrapper")]
  extern "C" {
    pub(super) fn init_system();
    pub(super) fn led_on(idx: u32);
  }
}

pub fn init_system() {
  unsafe {
    driverwrapper::init_system();
  }
}

// TODO: Add more peripherals

pub fn led_on(idx: u32) {
  unsafe {
    driverwrapper::led_on(idx);
  }
}
