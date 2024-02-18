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

//write to trasmit fifo (use of u8 becasue unsigned char)
//returns num bytes written
pub fn UART_write_FIFO(bytes: &mut[u8], len: u32) -> u32 {
  driverwrapper::UART_write_FIFO(*bytes, len);
}
//read from reciever fifo 
//returns num bytes read
pub fn UART_read_FIFO(bytes: &mut[u8], len: u32) -> u32 {
  let ret: u32 = unsafe{driverwrappper::UART_read_FIFO(*bytes, len)};
}
