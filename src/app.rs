#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [KPP])]
mod app {

    use crate::{gpio, run, usb};

    use teensy4_bsp::{
        board,
        hal::usbd::{BusAdapter, EndpointMemory, EndpointState, Speed},
    };
    use usb_device::{bus::UsbBusAllocator, device::UsbDevice};
    use usbd_midi::midi_device::MidiClass;

    #[local]
    struct Local {
        usb_device: UsbDevice<'static, BusAdapter>,
        midi: MidiClass<'static, BusAdapter>,
        sensors: [gpio::Sensor; 2],
        led: board::Led,
        is_usb_configured: bool,
    }

    #[shared]
    struct Shared {}

    #[init(local = [
        ep_memory: EndpointMemory<1024> = EndpointMemory::new(),
        ep_state: EndpointState = EndpointState::max_endpoints(),
        usb_bus: Option<UsbBusAllocator<BusAdapter>> = None,
    ])]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            usb, pins, gpio2, ..
        } = board::t41(cx.device);
        let (sensors, led) = gpio::init_gpio(gpio2, pins);
        let (midi, usb_device) = usb::init_usb(
            BusAdapter::with_speed(usb, cx.local.ep_memory, cx.local.ep_state, Speed::LowFull),
            cx.local.usb_bus,
        );
        {
            rtic_monotonics::systick::Systick::start(
                cx.core.SYST,
                board::ARM_FREQUENCY,
                rtic_monotonics::create_systick_token!(),
            );
        }
        run_midi_controller::spawn().unwrap();
        (
            Shared {},
            Local {
                usb_device,
                led,
                midi,
                is_usb_configured: false,
                sensors,
            },
        )
    }

    #[task(local = [
        led,
        usb_device,
        midi,
        is_usb_configured,
        sensors,
    ])]
    async fn run_midi_controller(cx: run_midi_controller::Context) {
        run::run(cx);
    }
}
