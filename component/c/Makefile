CC = arm-none-eabi-gcc
FLAGS = -mcpu=cortex-m4 -mthumb -mgeneral-regs-only -Wa,-mimplicit-it=thumb -O2 -ffreestanding -ffunction-sections -fdata-sections -Wall -MD -c -fno-isolate-erroneous-paths-dereference -g3 -ggdb -Wall -fdiagnostics-color=always
AR = arm-none-eabi-ar
default:
	@echo "Usage: make [clean|release]"
clean:
	@rm -rf build
release:
	@mkdir -p build
	$(CC) $(FLAGS) -c src/post_boot.c -o build/post_boot.o
	$(AR) rcs build/libpostboot.a build/post_boot.o
