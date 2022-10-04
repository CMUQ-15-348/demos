#include <stdint.h>
#include "15348.h"
#include "serial.h"
#include "i2c0.h"
#include "compass.h"

void PLLInit()
{
    SYSCTL_RCC2_R |= 0x80000000;
    SYSCTL_RCC2_R |= 0x00000800;
    SYSCTL_RCC_R = (SYSCTL_RCC_R & ~0x000007C0) + 0x00000540;
    SYSCTL_RCC2_R &= ~0x00000070;
    SYSCTL_RCC2_R &= ~0x00002000;
    SYSCTL_RCC2_R |= 0x40000000;
    SYSCTL_RCC2_R = (SYSCTL_RCC2_R & ~0x1FC00000) + (4 << 22);
    while ((SYSCTL_RIS_R & 0x00000040) == 0)
    {
    };
    SYSCTL_RCC2_R &= ~0x00000800;
}

// I2C address of the compass
#define ADDRESS 0x0D

// Register addresses
#define REG_XOUT_LSB 0x00
#define REG_STATUS 0x06
#define REG_CR1 0x09
#define REG_CR2 0x0A
#define REG_SR_PERIOD 0x0B

/**
 * main.c
 */
int main(void)
{
    PLLInit();
    SetupSerial();
    I2C_Init();

    SerialWriteLine("Go compass");

    Compass_Init();

    while (1)
    {

        Compass_Wait_Until_Available();

        // Read X, Y, Z (the current register auto-advances with each read)
        uint16_t x = Compass_getX();
        uint16_t y = Compass_getY();
        uint16_t z = Compass_getZ();

        SerialWrite("X: ");
        SerialWriteShortHex(x);

        SerialWrite("  Y: ");
        SerialWriteShortHex(y);

        SerialWrite("  Z: ");
        SerialWriteShortHex(z);

        SerialWriteLine("");
    }
}
