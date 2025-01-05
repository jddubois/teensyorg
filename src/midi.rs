use crate::gpio::Sensor;

use teensy4_bsp::hal::usbd::BusAdapter;
use usbd_midi::{
    data::{
        byte::u7::U7,
        midi::message::Message,
        usb_midi::{cable_number::CableNumber, usb_midi_event_packet::UsbMidiEventPacket},
    },
    midi_device::MidiClass,
};

pub fn send_message(midi: &mut MidiClass<BusAdapter>, sensor: &Sensor, value: bool) {
    midi.send_message(UsbMidiEventPacket::from_midi(
        CableNumber::Cable0,
        message_from_value(sensor, value),
    ))
    .unwrap();
}

fn message_from_value(sensor: &Sensor, value: bool) -> Message {
    if !value {
        Message::NoteOn(sensor.channel, sensor.note, U7::MAX)
    } else {
        Message::NoteOff(sensor.channel, sensor.note, U7::MAX)
    }
}
