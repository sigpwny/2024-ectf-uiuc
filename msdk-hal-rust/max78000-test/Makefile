debug:
	cargo build
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/max78000-test max78000-test-debug.bin
	python3 package.py
release:
	cargo build --release
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/max78000-test max78000-test-release.bin