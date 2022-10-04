#include <stdint.h>
#include "15348.h"
#include "serial.h"
#include "i2c0.h"
#include "screen.h"

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

    setTextXY(0, 0);
    putString("Hi there");

    while(1) {

    }
}
