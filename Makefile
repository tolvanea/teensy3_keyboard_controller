BIN=teensy3-rs-demo
OUTDIR=target/thumbv7em-none-eabi/release
HEX=$(OUTDIR)/$(BIN).hex
ELF=$(OUTDIR)/$(BIN)
# The CPU type is written to temporal file by build.rs
MCU=$(<$(OUTDIR)/mcu_info.txt)


all:: $(ELF)

.PHONY: $(ELF)
$(ELF):
	cross build --target thumbv7em-none-eabi --release
	#cargo build --release

$(HEX): $(ELF)
	arm-none-eabi-objcopy -O ihex $(ELF) $(HEX)

# TODO FIGURE OUT TO GET MCU FROM UGLY OUT PATH
#.PHONY: flash
#flash: $(HEX)
#	teensy_loader_cli -w -mmcu=$(MCU) $(HEX) -v

# arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabi/release/teensy3-rs-demo target/thumbv7em-none-eabi/release/teensy3-rs-demo.hex
# teensy_loader_cli -w -mmcu=TEENSY36 target/thumbv7em-none-eabi/release/teensy3-rs-demo.hex -v

# arm-none-eabi-objcopy -O ihex -R .eeprom target/thumbv7em-none-eabi/release/teensy3-rs-demo target/thumbv7em-none-eabi/release/teensy3-rs-demo.hex
# teensy_loader_cli -w -s --mcu=TEENSY36 target/thumbv7em-none-eabi/release/teensy3-rs-demo.hex
