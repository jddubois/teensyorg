use crate::sensor::{Sensor, SensorPin};

use imxrt_hal::{gpio::Port, iomuxc};
use rtic_monotonics::{systick::Systick, Monotonic};
use teensy4_bsp::{
    board,
    pins::{self, t41::Pins},
};

pub fn init_gpio(
    mut gpio1: Port<1>,
    mut gpio2: Port<2>,
    mut gpio3: Port<3>,
    mut gpio4: Port<4>,
    mut pins: Pins,
) -> ([Sensor; 32], board::Led) {
    const PIN_CONFIG: iomuxc::Config = iomuxc::Config::zero()
        .set_pull_keeper(Some(iomuxc::PullKeeper::Pullup100k))
        .set_drive_strength(iomuxc::DriveStrength::R0)
        .set_speed(iomuxc::Speed::Low)
        .set_slew_rate(iomuxc::SlewRate::Slow)
        .set_hysteresis(pins::Hysteresis::Enabled);

    iomuxc::configure(&mut pins.p0, PIN_CONFIG);
    iomuxc::configure(&mut pins.p1, PIN_CONFIG);
    iomuxc::configure(&mut pins.p2, PIN_CONFIG);
    iomuxc::configure(&mut pins.p3, PIN_CONFIG);
    iomuxc::configure(&mut pins.p4, PIN_CONFIG);
    iomuxc::configure(&mut pins.p5, PIN_CONFIG);
    iomuxc::configure(&mut pins.p6, PIN_CONFIG);
    iomuxc::configure(&mut pins.p7, PIN_CONFIG);
    iomuxc::configure(&mut pins.p8, PIN_CONFIG);
    iomuxc::configure(&mut pins.p9, PIN_CONFIG);
    iomuxc::configure(&mut pins.p10, PIN_CONFIG);
    iomuxc::configure(&mut pins.p11, PIN_CONFIG);
    iomuxc::configure(&mut pins.p12, PIN_CONFIG);
    iomuxc::configure(&mut pins.p14, PIN_CONFIG);
    iomuxc::configure(&mut pins.p15, PIN_CONFIG);
    iomuxc::configure(&mut pins.p16, PIN_CONFIG);
    iomuxc::configure(&mut pins.p17, PIN_CONFIG);
    iomuxc::configure(&mut pins.p18, PIN_CONFIG);
    iomuxc::configure(&mut pins.p19, PIN_CONFIG);
    iomuxc::configure(&mut pins.p20, PIN_CONFIG);
    iomuxc::configure(&mut pins.p21, PIN_CONFIG);
    iomuxc::configure(&mut pins.p22, PIN_CONFIG);
    iomuxc::configure(&mut pins.p23, PIN_CONFIG);
    iomuxc::configure(&mut pins.p24, PIN_CONFIG);
    iomuxc::configure(&mut pins.p25, PIN_CONFIG);
    iomuxc::configure(&mut pins.p26, PIN_CONFIG);
    iomuxc::configure(&mut pins.p27, PIN_CONFIG);
    iomuxc::configure(&mut pins.p28, PIN_CONFIG);
    iomuxc::configure(&mut pins.p29, PIN_CONFIG);
    iomuxc::configure(&mut pins.p30, PIN_CONFIG);
    iomuxc::configure(&mut pins.p31, PIN_CONFIG);
    iomuxc::configure(&mut pins.p32, PIN_CONFIG);

    let sensors = [
        Sensor {
            pin: SensorPin::P0(gpio1.input(pins.p0)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::C2,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P1(gpio1.input(pins.p1)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::Cs2,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P2(gpio4.input(pins.p2)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::D2,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P3(gpio4.input(pins.p3)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::Ds2,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P4(gpio4.input(pins.p4)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::E2,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P5(gpio4.input(pins.p5)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::F2,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P6(gpio2.input(pins.p6)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::Fs2,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P7(gpio2.input(pins.p7)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::G2,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P8(gpio2.input(pins.p8)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::Gs2,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P9(gpio2.input(pins.p9)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::A2,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P10(gpio2.input(pins.p10)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::As2,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P11(gpio2.input(pins.p11)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::B2,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P12(gpio2.input(pins.p12)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::C3,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P14(gpio1.input(pins.p14)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::Cs3,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        // TODO
        Sensor {
            pin: SensorPin::P15(gpio1.input(pins.p15)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::D3,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P16(gpio1.input(pins.p16)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::Ds3,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P17(gpio1.input(pins.p17)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::E3,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P18(gpio1.input(pins.p18)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::F3,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P19(gpio1.input(pins.p19)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::Fs3,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P20(gpio1.input(pins.p20)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::G3,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P21(gpio1.input(pins.p21)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::Gs3,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P22(gpio1.input(pins.p22)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::A3,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P23(gpio1.input(pins.p23)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::As3,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P24(gpio1.input(pins.p24)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::B3,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P25(gpio1.input(pins.p25)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::C4,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P26(gpio1.input(pins.p26)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::Cs4,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P27(gpio1.input(pins.p27)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::D4,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P28(gpio3.input(pins.p28)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::Ds4,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P29(gpio4.input(pins.p29)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::E4,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P30(gpio3.input(pins.p30)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::F4,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P31(gpio3.input(pins.p31)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::Fs4,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P32(gpio2.input(pins.p32)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::G4,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
    ];

    (sensors, board::led(&mut gpio2, pins.p13))
}
