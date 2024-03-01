# BUILD_DIR = $(abspath build)
# export BUILD_DIR
CC = arm-none-eabi-gcc
#FLAGS = -mcpu=cortex-m4 -mthumb -mfloat-abi=hard -mfpu=fpv4-sp-d16 -Wa,-mimplicit-it=thumb -O2 -ffunction-sections -fdata-sections -Wall -Wno-format -MD -c -fno-isolate-erroneous-paths-dereference -g3 -ggdb -Wall -fdiagnostics-color=always -Iinc
FLAGS = -mcpu=cortex-m4 -mthumb -mgeneral-regs-only -Wa,-mimplicit-it=thumb -O2 -ffunction-sections -fdata-sections -Wall -Wno-format -MD -c -fno-isolate-erroneous-paths-dereference -g3 -ggdb -Wall -fdiagnostics-color=always -Iinc
AR = arm-none-eabi-ar
default:
	@echo "Usage: make [clean|release]"
clean:
	@rm -rf build
# release:
# 	@${MAKE} -f msdk.mk lib
# 	@cp build/max78000.a build/libpostboot.a
# 	@cp $(MAXIM_PATH)/Libraries/PeriphDrivers/bin/MAX78000/softfp/libPeriphDriver_softfp.a build/libmsdk-periph.a
# 	@ar rcs build/libmsdk-board.a build/board.o
# 	@ar d build/libpostboot.a project.mk
release:
	@mkdir -p build
#	@mkdir -p inc
#	@cp ../../rust-lib/hal-ffi/halffi.h inc/halffi.h
#	@echo "TODO: Build the HAL FFI library"
#	@cp ../../rust-lib/hal-ffi/target/release/libhalffi.a build/libpostboot.a
	$(CC) $(FLAGS) -c src/fixes.c -o build/fixes.o
	$(CC) $(FLAGS) -c src/stdio.c -o build/stdio.o
	$(CC) $(FLAGS) -c src/post_boot.c -o build/post_boot.o
	$(AR) rcs build/libpostboot.a build/post_boot.o build/stdio.o build/fixes.o
#	$(CC) -o build/libpostbootandlibc.so --shared build/libpostboot.a $(GCC_ARM_PATH)/arm-none-eabi/lib/thumb/v7e-m+fp/hard/libc.a
