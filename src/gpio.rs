use imxrt_hal::{gpio::Port, iomuxc};
use rtic_monotonics::{systick::Systick, Monotonic};

type SystickInstant = <Systick as Monotonic>::Instant;

use teensy4_bsp::{
    board,
    hal::gpio,
    pins::{self, t41::Pins},
};

pub enum SensorPin {
    P7(gpio::Input<pins::t41::P7>),
    P8(gpio::Input<pins::t41::P8>),
}

impl SensorPin {
    pub fn is_set(&self) -> bool {
        match self {
            SensorPin::P7(pin) => pin.is_set(),
            SensorPin::P8(pin) => pin.is_set(),
        }
    }
}

pub struct Sensor {
    pub pin: SensorPin,
    pub stable_value: bool,
    pub previous_change_time: Option<SystickInstant>,
    pub note: usbd_midi::data::midi::notes::Note,
    pub channel: usbd_midi::data::midi::channel::Channel,
}

pub fn init_gpio(mut gpio2: Port<2>, mut pins: Pins) -> ([Sensor; 2], board::Led) {
    const PIN_CONFIG: iomuxc::Config = iomuxc::Config::zero()
        .set_pull_keeper(Some(iomuxc::PullKeeper::Pullup100k))
        .set_drive_strength(iomuxc::DriveStrength::R0)
        .set_speed(iomuxc::Speed::Low)
        .set_slew_rate(iomuxc::SlewRate::Slow)
        .set_hysteresis(pins::Hysteresis::Enabled);

    iomuxc::configure(&mut pins.p7, PIN_CONFIG);
    iomuxc::configure(&mut pins.p8, PIN_CONFIG);

    let sensors = [
        Sensor {
            pin: SensorPin::P7(gpio2.input(pins.p7)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::A5,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
        Sensor {
            pin: SensorPin::P8(gpio2.input(pins.p8)),
            stable_value: true,
            previous_change_time: None,
            note: usbd_midi::data::midi::notes::Note::C5,
            channel: usbd_midi::data::midi::channel::Channel::Channel1,
        },
    ];

    (sensors, board::led(&mut gpio2, pins.p13))
}
