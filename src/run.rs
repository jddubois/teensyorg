use crate::{app::app::run_midi_controller, debounce, midi, usb};

use rtic_monotonics::{
    systick::{ExtU32, Systick},
    Monotonic,
};

pub fn run(cx: run_midi_controller::Context) {
    loop {
        usb::poll(
            cx.local.usb_device,
            cx.local.midi,
            cx.local.is_usb_configured,
        );

        if !*cx.local.is_usb_configured {
            cx.local.led.clear();
            continue;
        } else {
            cx.local.led.set();
        }

        for sensor in cx.local.sensors.iter_mut() {
            let current_value = sensor.pin.is_set();
            let did_value_change = debounce::debounce_input(
                current_value,
                &mut sensor.stable_value,
                Systick::now(),
                &mut sensor.previous_change_time,
                5.millis(),
            );
            if did_value_change {
                midi::send_message(cx.local.midi, sensor, current_value);
            }
        }
    }
}
