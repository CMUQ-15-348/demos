/*
 * serial.c
 *
 *  Created on: Feb 12, 2017
 *      Author: srazak
 *  Edited on: Sep 16, 2021
 *      Author: efeoflus
 *  Edited on: Sep 28, 2022
 *      Author: rdriley
 */
#include <stdint.h>
#include "15348.h"

void SetupSerial()
{
    SYSCTL_RCGCUART_R |= 0x01;
    SYSCTL_RCGCGPIO_R |= 0x01;

    //GPIO_PORTA_CR_R = 0xFF;
    GPIO_PORTA_AFSEL_R |= 0x03;
    GPIO_PORTA_DEN_R |= 0x03;
    GPIO_PORTA_PCTL_R = 0x11;

    UART0_CTL_R &= ~UART_CTL_UARTEN;

    // I don't know why the calculations are done based on 16 MHz instead of 80Mhz.
    UART0_IBRD_R = 8;  // 16,000,000 / (16*115200) == 8.680555555555555
    UART0_FBRD_R = 44; // int(0.6805*64 + 0.5) == 44
    // This doesn't work, but I think it should.
    //UART0_IBRD_R = 43;  // 80,000,000 / (16*115200) == 43.40277777777778
    //UART0_FBRD_R = 26; // int(0.402777777*64 + 0.5) == 26

    UART0_LCRH_R = 0x0070;
    UART0_CTL_R |= UART_CTL_UARTEN;
    UART0_IFLS_R = 0x12;
    UART0_CC_R = 0x05;

}
void SerialWrite(char *st)
{
    char *temp = st;
    while (*temp != 0)
    {
        while (UART0_FR_R & 0x20)
            ;
        UART0_DR_R = *temp;
        temp++;
        asm("    NOP");
        asm("    NOP");
    }

}
void SerialWriteLine(char *st)
{
    SerialWrite(st);
    while (UART0_FR_R & 0x20)
        ;
    UART0_DR_R = 13;
    __asm("    NOP");
    __asm("    NOP");
    while (UART0_FR_R & 0x20)
        ;
    UART0_DR_R = 10;
    __asm("    NOP");
    __asm("    NOP");

}
void SerialWriteInt(int n)
{
    // change n to String
    char str[] = { '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', 0 };
    int i = 9;
    while (n > 0)
    {
        str[i] = '0' + n % 10;
        n = n / 10;
        i--;
    }
    char *ch = str;
    while (*ch != 0 && *ch == '0')
        ch++;
    SerialWriteLine(ch);
}

static char hexmap[] = { '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A',
                         'B', 'C', 'D', 'E', 'F' };

void SerialWriteHex(unsigned int n)
{
    char str[] = { '0', '0', '0', '0', '0', '0', '0', '0', 0 };
    int i = 7;
    while (n > 0)
    {
        str[i] = hexmap[n & 0xf];
        n >>= 4;
        i--;
    }
    SerialWriteLine(str);
}

void SerialWriteShortHex(uint16_t n)
{
    char str[] = { '0', '0', '0', '0', 0 };
    int i = 3;
    while (n > 0)
    {
        str[i] = hexmap[n & 0xf];
        n >>= 4;
        i--;
    }
    SerialWrite(str);
}

