default:
	@echo "Usage: make [clean|release]"
clean:
	@echo "Cleaning AP"
	@poetry run python3 preprocess_ap.py --clean
	@${MAKE} -C c/ clean
	@${MAKE} -C rust/ clean
	@rm -rf build
release:
	@echo "Building release firmware"
	@poetry run python3 preprocess_ap.py
	@${MAKE} -C c/ release
	@${MAKE} -C rust/ release
	@mkdir -p build
	@cp rust/build/max78000.elf build/
	@cp rust/build/max78000.bin build/
