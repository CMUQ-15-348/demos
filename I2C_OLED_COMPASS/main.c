#include <stdint.h>
#include "15348.h"
#include "serial.h"
#include "i2c0.h"
#include "screen.h"
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

void align_number(uint16_t num)
{
    if (num < 10) {
        putString("    ");
    } else if (num < 100) {
        putString("   ");
    } else if (num < 1000) {
        putString("  ");
    } else if (num < 10000) {
        putString(" ");
    }
}

/**
 * main.c
 */
int main(void)
{
    PLLInit();
    SetupSerial();
    I2C_Init();

    SerialWriteLine("Go screen/compass");

    initDisplay();
    Compass_Init();

    setTextXY(0, 0);
    putString("Compass:");

    while(1) {
        //clearDisplay();

        Compass_Wait_Until_Available();

        // Read X, Y, Z (the current register auto-advances with each read)
        uint16_t x = Compass_getX();
        uint16_t y = Compass_getY();
        uint16_t z = Compass_getZ();

        SerialWrite("X: ");
        SerialWriteShortHex(x);
        setTextXY(2, 0);
        putString("X: ");
        align_number(x);
        putNumber(x);

        SerialWrite("  Y: ");
        SerialWriteShortHex(y);
        setTextXY(3, 0);
        putString("Y: ");
        align_number(y);
        putNumber(y);

        SerialWrite("  Z: ");
        SerialWriteShortHex(z);
        setTextXY(4, 0);
        putString("Z: ");
        align_number(z);
        putNumber(z);

        SerialWriteLine("");
    }
}
