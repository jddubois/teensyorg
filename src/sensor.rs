use rtic_monotonics::{systick::Systick, Monotonic};
use teensy4_bsp::{hal::gpio, pins};

type SystickInstant = <Systick as Monotonic>::Instant;

pub enum SensorPin {
    P0(gpio::Input<pins::t41::P0>),
    P1(gpio::Input<pins::t41::P1>),
    P2(gpio::Input<pins::t41::P2>),
    P3(gpio::Input<pins::t41::P3>),
    P4(gpio::Input<pins::t41::P4>),
    P5(gpio::Input<pins::t41::P5>),
    P6(gpio::Input<pins::t41::P6>),
    P7(gpio::Input<pins::t41::P7>),
    P8(gpio::Input<pins::t41::P8>),
    P9(gpio::Input<pins::t41::P9>),
    P10(gpio::Input<pins::t41::P10>),
    P11(gpio::Input<pins::t41::P11>),
    P12(gpio::Input<pins::t41::P12>),
    P14(gpio::Input<pins::t41::P14>),
    P15(gpio::Input<pins::t41::P15>),
    P16(gpio::Input<pins::t41::P16>),
    P17(gpio::Input<pins::t41::P17>),
    P18(gpio::Input<pins::t41::P18>),
    P19(gpio::Input<pins::t41::P19>),
    P20(gpio::Input<pins::t41::P20>),
    P21(gpio::Input<pins::t41::P21>),
    P22(gpio::Input<pins::t41::P22>),
    P23(gpio::Input<pins::t41::P23>),
    P24(gpio::Input<pins::t41::P24>),
    P25(gpio::Input<pins::t41::P25>),
    P26(gpio::Input<pins::t41::P26>),
    P27(gpio::Input<pins::t41::P27>),
    P28(gpio::Input<pins::t41::P28>),
    P29(gpio::Input<pins::t41::P29>),
    P30(gpio::Input<pins::t41::P30>),
    P31(gpio::Input<pins::t41::P31>),
    P32(gpio::Input<pins::t41::P32>),
}

impl SensorPin {
    pub fn is_set(&self) -> bool {
        match self {
            SensorPin::P0(pin) => pin.is_set(),
            SensorPin::P1(pin) => pin.is_set(),
            SensorPin::P2(pin) => pin.is_set(),
            SensorPin::P3(pin) => pin.is_set(),
            SensorPin::P4(pin) => pin.is_set(),
            SensorPin::P5(pin) => pin.is_set(),
            SensorPin::P6(pin) => pin.is_set(),
            SensorPin::P7(pin) => pin.is_set(),
            SensorPin::P8(pin) => pin.is_set(),
            SensorPin::P9(pin) => pin.is_set(),
            SensorPin::P10(pin) => pin.is_set(),
            SensorPin::P11(pin) => pin.is_set(),
            SensorPin::P12(pin) => pin.is_set(),
            SensorPin::P14(pin) => pin.is_set(),
            SensorPin::P15(pin) => pin.is_set(),
            SensorPin::P16(pin) => pin.is_set(),
            SensorPin::P17(pin) => pin.is_set(),
            SensorPin::P18(pin) => pin.is_set(),
            SensorPin::P19(pin) => pin.is_set(),
            SensorPin::P20(pin) => pin.is_set(),
            SensorPin::P21(pin) => pin.is_set(),
            SensorPin::P22(pin) => pin.is_set(),
            SensorPin::P23(pin) => pin.is_set(),
            SensorPin::P24(pin) => pin.is_set(),
            SensorPin::P25(pin) => pin.is_set(),
            SensorPin::P26(pin) => pin.is_set(),
            SensorPin::P27(pin) => pin.is_set(),
            SensorPin::P28(pin) => pin.is_set(),
            SensorPin::P29(pin) => pin.is_set(),
            SensorPin::P30(pin) => pin.is_set(),
            SensorPin::P31(pin) => pin.is_set(),
            SensorPin::P32(pin) => pin.is_set(),
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
