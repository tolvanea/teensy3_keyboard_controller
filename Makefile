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

TARGET=thumbv7em-none-eabi
# Enable hard floating point with for teensy 3.5 and 3.6 by uncommenting following line.
# However, it is not yet working. If you want to fight linker errors, then go ahead.
# I bet the problem is in old versions of newlib that ubuntu 16.04 uses
TARGET=thumbv7em-none-eabihf


BIN=teensy3-rs-demo
OUTDIR=target/$(TARGET)/release
HEXPATH=$(OUTDIR)/$(BIN).hex
BINPATH=$(OUTDIR)/$(BIN)

all:: $(BINPATH)

.PHONY: $(BINPATH)
$(BINPATH):
	cross build --release --target $(TARGET) --features "$(MODEL)"

.PHONY: debug
debug:
	cross build --target $(TARGET) --features "$(MODEL)" --verbose

.PHONY: doc
doc:
	cross doc --features TEENSY36 --target "$(TARGET)"

$(HEXPATH): $(BINPATH)
	arm-none-eabi-objcopy -O ihex -R .eeprom $(BINPATH) $(HEXPATH)

.PHONY: flash
flash: $(HEXPATH)
	teensy_loader_cli -w -s --mcu=$(MODEL) $(HEXPATH) -v

.PHONY: clean
clean:
	cross clean
