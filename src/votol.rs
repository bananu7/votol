use binary_layout::prelude::*;

binary_layout!(votol_data_packet, BigEndian, {
  header: u16,
  reserved_a: [u8; 3],
  battery_voltage: u16,
  battery_current: u16,
  reserved_b: u8,
  faults: u32,
  rpm: u16,
  controller_temp: u8, // starts at -50C
  outside_temp: u8,    // subtract 50 to get proper value
  flags: u8,
  status: u8,
  checksum: u8,
});

fn func(packet_data: &mut [u8]) {
  let mut view = votol_data_packet::View::new(packet_data);
  //let flags: u8 = view.flags().read();
  // equivalent: let code: u8 = packet_data[1];
}
