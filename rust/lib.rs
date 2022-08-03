// import the flatbuffers runtime library
extern crate flatbuffers;

// import the generated code
#[allow(dead_code, unused_imports)]
pub mod pod_state_generated;
pub use pod_state_generated as pod_state;

#[allow(dead_code, unused_imports)]
pub mod pod_data_generated;
pub use pod_data_generated as pod_data;

#[allow(dead_code, unused_imports)]
pub mod udp_desktop_messages_generated;
pub use udp_desktop_messages_generated as udp_desktop_message;

#[allow(dead_code, unused_imports)]
pub mod udp_pod_state_message_generated;
pub use udp_pod_state_message_generated as udp_pod_state_message;

#[allow(dead_code, unused_imports)]
pub mod error_types_generated;

/// Test Module for the flatbuffers, also shows example usages
mod test {
  use super::*;
 extern crate flatbuffers;
  #[test]
  fn test_udp_desktop_message_explicit_creation() {
    let mut fbb = flatbuffers::FlatBufferBuilder::with_capacity(128);
    let mut udp_message_builder = udp_desktop_message::UdpDesktopMessageBuilder::new(&mut fbb);
    udp_message_builder.add_most_recent_timestamp(1000);
    udp_message_builder.add_requested_state(pod_state::PodState::LowVoltage);
    let udp_message_offset = udp_message_builder.finish();
    fbb.finish(udp_message_offset, None);
    let udp_message_bytes = fbb.finished_data();
    // THis is where data would normally go onto the wire
    let udp_message = udp_desktop_message::root_as_udp_desktop_message(udp_message_bytes).unwrap();
    assert_eq!(udp_message.most_recent_timestamp(), 1000);
    assert_eq!(udp_message.requested_state(), pod_state::PodState::LowVoltage);
  }

  #[test]
  fn test_udp_desktop_message_fast_creation() {
    let mut fbb = flatbuffers::FlatBufferBuilder::with_capacity(128);
    let udp_message_offset = udp_desktop_message::UdpDesktopMessage::create(
      &mut fbb,
      &udp_desktop_message::UdpDesktopMessageArgs {
        requested_state: pod_state::PodState::LowVoltage,
        most_recent_timestamp: 1000
      }
    );
    fbb.finish(udp_message_offset, None);
    let udp_message_bytes = fbb.finished_data();
    // THis is where data would normally go onto the wire
    let udp_message = udp_desktop_message::root_as_udp_desktop_message(udp_message_bytes).unwrap();
    assert_eq!(udp_message.most_recent_timestamp(), 1000);
    assert_eq!(udp_message.requested_state(), pod_state::PodState::LowVoltage);
  }
}
