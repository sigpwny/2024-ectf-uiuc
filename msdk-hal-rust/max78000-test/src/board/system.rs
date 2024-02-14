use cortex_m::peripheral::scb;

fn configure_vtor(scb: &scb::RegisterBlock, vtor: u32) {
  unsafe {
    scb.vtor.write(vtor);
  }
}