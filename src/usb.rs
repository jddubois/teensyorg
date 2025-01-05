use teensy4_bsp::hal::usbd::BusAdapter;
use usb_device::{
    bus::UsbBusAllocator,
    device::{UsbDevice, UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
};
use usbd_midi::midi_device::MidiClass;

const VID_PID: UsbVidPid = UsbVidPid(0x16c0, 0x5e4);
const PRODUCT: &str = "teensyorg";

pub fn init_usb(
    bus_adapter: BusAdapter,
    out_usb_bus: &mut Option<UsbBusAllocator<BusAdapter>>,
) -> (MidiClass<BusAdapter>, UsbDevice<BusAdapter>) {
    bus_adapter.set_interrupts(true);
    let usb_bus = out_usb_bus.insert(UsbBusAllocator::new(bus_adapter));
    let midi = MidiClass::new(usb_bus);
    let usb_device = UsbDeviceBuilder::new(usb_bus, VID_PID)
        .product(PRODUCT)
        .device_class(0)
        .device_sub_class(0)
        .build();

    (midi, usb_device)
}

pub fn poll(
    usb_device: &mut UsbDevice<'_, BusAdapter>,
    midi: &mut MidiClass<'_, BusAdapter>,
    is_usb_configured: &mut bool,
) {
    if usb_device.poll(&mut [midi]) {
        if usb_device.state() == UsbDeviceState::Configured {
            if !*is_usb_configured {
                usb_device.bus().configure();
            }
            *is_usb_configured = true;
        } else {
            *is_usb_configured = false;
        }
    }
}
