# Choose your model below by uncommenting the corresponding line.
#MODEL=TEENSY30
#MODEL=TEENSY31
#MODEL=TEENSY32
#MODEL=TEENSY35
MODEL=TEENSY36

# Following definition makes hopefully more understandable error message when
# model is not specified.
ifndef MODEL
    MODEL=PLEASE_SPECIFY_YOUR_TEENSY_MODEL_IN_MAKEFILE
endif


BIN=teensy3-rs-demo
OUTDIR=target/thumbv7em-none-eabi/release
HEXPATH=$(OUTDIR)/$(BIN).hex
BINPATH=$(OUTDIR)/$(BIN)

all:: $(BINPATH)

.PHONY: $(BINPATH)
$(BINPATH):
	cross build --release --target thumbv7em-none-eabi --features "$(MODEL)"

.PHONY: debug
debug:
	cross build --debug --target thumbv7em-none-eabi --features "$(MODEL)"

$(HEXPATH): $(BINPATH)
	arm-none-eabi-objcopy -O ihex -R .eeprom $(BINPATH) $(HEXPATH)

.PHONY: flash
flash: $(HEXPATH)
	teensy_loader_cli -w -s --mcu=$(MODEL) $(HEXPATH) -v

.PHONY: clean
clean:
	cross clean
