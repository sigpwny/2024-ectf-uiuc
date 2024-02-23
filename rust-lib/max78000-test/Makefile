BIN_NAME = max78000-test

default:
	@echo "Usage: make [debug|semihosting|release|clean]"
debug:
	mkdir -p build
	cargo build
	cp target/thumbv7em-none-eabihf/debug/${BIN_NAME} build/max78000-debug.elf
	arm-none-eabi-objcopy -O binary build/max78000-debug.elf build/max78000-debug.bin
	python3 package.py
semihosting:
	mkdir -p build
	cargo build --features semihosting
	cp target/thumbv7em-none-eabihf/debug/${BIN_NAME} build/max78000-debug.elf
	arm-none-eabi-objcopy -O binary build/max78000-debug.elf build/max78000-debug.bin
	python3 package.py
release:
	mkdir -p build
	cargo build --release
	cp target/thumbv7em-none-eabihf/release/${BIN_NAME} build/max78000.elf
	arm-none-eabi-objcopy -O binary build/max78000.elf build/max78000.bin
	python3 package.py # TODO: Add flash secrets
clean:
	rm -rf build
	cargo clean