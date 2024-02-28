default:
	@echo "Usage: make [clean|release]"
clean:
	@echo "Cleaning AP"
	@poetry run python3 preprocess_ap.py --clean
	@${MAKE} -C c/ clean
	@${MAKE} -C rust/ clean
release:
	@echo "Building release firmware"
	@poetry run python3 preprocess_ap.py
	@${MAKE} -C c/ release
	@${MAKE} -C rust/ release
