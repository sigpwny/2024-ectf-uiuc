mod driverwrapper {
  #[link(name = "driverwrapper")]
  extern "C" {
      pub(super) fn init_system();
  }
}

pub fn init_system() {
  unsafe {
      driverwrapper::init_system();
  }
}

// TODO: Add more peripherals
