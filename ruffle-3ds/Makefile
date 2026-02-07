# Ruffle 3DS - One-command build
# Usage: make

BINARY_NAME := ruffle-3ds
TARGET := armv6k-nintendo-3ds
RELEASE_DIR := target/$(TARGET)/release

.PHONY: all clean

all: $(BINARY_NAME).3dsx

$(RELEASE_DIR)/$(BINARY_NAME): FORCE
	@echo "=== Building for 3DS ==="
	cargo +nightly build --release --target $(TARGET) -Zbuild-std

$(BINARY_NAME).3dsx: $(RELEASE_DIR)/$(BINARY_NAME)
	@echo "=== Converting ELF to 3DSX ==="
	3dsxtool $< $@
	@echo "=== Build complete: $@ ==="

clean:
	cargo clean
	rm -f $(BINARY_NAME).3dsx

FORCE:
