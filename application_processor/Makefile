default:
	@echo "default"
release:
	@echo "Building release firmware"
	python3 preprocess_ap.py
	${MAKE} -C c/ release
	${MAKE} -C rust/ release