# Makefile: Based on "Life with David-BMA04"
# https://github.com/LifeWithDavid/RaspberryPiPico-BareMetalAdventures

.PHONY: all clean run

CC=arm-none-eabi-gcc
MACH=cortex-m0plus
CFLAGS= -c -mcpu=$(MACH) -mthumb -std=gnu11 -Wall -O0 
LFLAGS= -nostdlib -T memmap.ld -Wl,-Map=final.map

all: bs2.o main.o vector_table.o reset.o

main.o: main.S
	$(CC) $(CFLAGS) -o $@ $^

bs2.o: bs2.S	
	$(CC) $(CFLAGS) -o $@ $^
	
vector_table.o: vector_table.S
	$(CC) $(CFLAGS) -o $@ $^
	
reset.o: reset.S
	$(CC) $(CFLAGS) -o $@ $^

main.elf: bs2.o vector_table.o main.o reset.o
	$(CC) $(LFLAGS) -o $@ $^

run: main.elf
	@echo "Flashing and running program.  Press ctrl-c to quit"
	probe-rs run --chip RP2040 --protocol swd main.elf

clean:
	-rm -f $(wildcard *.o)
	-rm -f $(wildcard *.elf)	
	-rm -f $(wildcard *.map)
