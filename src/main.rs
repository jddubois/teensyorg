#![no_std]
#![no_main]

use teensy4_panic as _;

mod app;
mod debounce;
mod gpio;
mod midi;
mod run;
mod sensor;
mod usb;
