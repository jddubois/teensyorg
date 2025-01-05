# teensyorg

teensyorg is a pipe organ MIDI controller for the Teensy 4.1 microcontroller

## Running

To build and load teensyorg onto your teensy, run:

`cargo objcopy -v --release -- -O ihex teensyorg.hex && sudo teensy_loader_cli -w -v --mcu=TEENSY41 teensyorg.hex`