CC = arm-none-eabi-gcc
AS = as
AR = arm-none-eabi-ar
LD = arm-none-eabi-gcc
TARGET = MAX78000
BUILD_DIR = build

CFLAGS = -mthumb -mcpu=cortex-m4 -mgeneral-regs-only -Wa,-mimplicit-it=thumb -O2 -ffunction-sections -fdata-sections -Wall -Wno-format -MD -c -fno-isolate-erroneous-paths-dereference -Wall -MD -fdiagnostics-color=always -I.
AFLAGS = -mthumb -mcpu=cortex-m4 -MD

default:
	mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -c -o $(BUILD_DIR)/aead.o aead.c
	$(CC) $(CFLAGS) -c -o $(BUILD_DIR)/crypto_aead.o crypto_aead.c
	$(CC) $(CFLAGS) -c -o $(BUILD_DIR)/permutations.o permutations.c
	$(CC) $(CFLAGS) -c -o $(BUILD_DIR)/printstate.o printstate.c
	$(CC) $(CFLAGS) -c -o $(BUILD_DIR)/update.o update.c
	# Create the library with all the object files
	${AR} rcs $(BUILD_DIR)/libascon.a $(BUILD_DIR)/*.o
clean:
	rm -rf $(BUILD_DIR)