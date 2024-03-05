BIN_NAME = ap

default:
	@echo "Usage: make [debug|semihosting|release|clean]"
debug:
	@mkdir -p build
	@cargo build
	@cp target/thumbv7em-none-eabi/debug/${BIN_NAME} build/max78000-debug.elf
	@arm-none-eabi-objcopy -O binary build/max78000-debug.elf build/max78000-debug.bin
	@poetry run python3 package.py --bin build/max78000-debug.bin --img build/max78000-debug.img
semihosting:
	@mkdir -p build
	@cargo build --features semihosting
	@cp target/thumbv7em-none-eabi/debug/${BIN_NAME} build/max78000-debug.elf
	@arm-none-eabi-objcopy -O binary build/max78000-debug.elf build/max78000-debug.bin
	@poetry run python3 package.py --bin build/max78000-debug.bin --img build/max78000-debug.img
release:
	@mkdir -p build
	@cargo build --release
	@cp target/thumbv7em-none-eabi/release/${BIN_NAME} build/max78000.elf
	@arm-none-eabi-objcopy -O binary build/max78000.elf build/max78000.bin
	@echo "IMPORTANT: You MUST perform a power-on reset in order to flash new firmware after you have flashed a release build!"
clean:
	@rm -rf build
	@cargo clean
