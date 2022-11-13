/*
 * uart.c
 *
 *  Created on: Oct 5, 2022
 *      Author: ryan
 */

#include <stdint.h>
#include "15348.h"
#include "serial.h"

// Rx on the TM4C is PE0 (Connect to Tx on the ESP)
// Tx on the TM4C is PE1 (Connect to Rx on the ESP)
void InitUART7()
{
    // Enable UART7
    SYSCTL_RCGCUART_R |= 0x80;
    // Activate Port E
    SYSCTL_RCGCGPIO_R |= 0x10;

    // Disable UART7
    UART7_CTL_R &= ~UART_CTL_UARTEN;

    // Set speed to 115200
    UART7_IBRD_R = 43; // 80,000,000 / (16*115200) == 43.40277777777778
    UART7_FBRD_R = 26; // int(0.402777777*64 + 0.5) == 26

    // 8-bit word length, enable FIFO
    UART7_LCRH_R = 0x0070;

    // Setup PORTE pins PE0, PE1
    GPIO_PORTE_PCTL_R = (GPIO_PORTE_PCTL_R & 0xFFFFFF00) | 0x11; // Set the alternate functionality we want: UART
    GPIO_PORTE_AMSEL_R &= ~0x03; // disable analog
    GPIO_PORTE_AFSEL_R |= 0x03; // enable alt functionality
    GPIO_PORTE_DEN_R |= 0x03; // enable digital IO

    // Interrupts
    UART7_IFLS_R |= 0x00; // Trigger interrupts early
    UART7_IM_R |= 0x50; // unmask RXIM (Receive mask) and RTIM (Receive time-out mask).  See page 931/932 in datasheet for details
    NVIC_EN1_R = ((uint32_t) 1 << (63 - 32));     // Enable interrupt 63

    // Enable RXE, TXE, and UART
    UART7_CTL_R |= (UART_CTL_UARTEN | UART_CTL_RXE | UART_CTL_TXE);
}

char UART7_InChar(void)
{
    while ((UART7_FR_R & 0x0010) != 0)
        ; // Wait until RXFE is 0 (data is available)
    return ((char) (UART7_DR_R & 0xFF));
}

void UART7_OutChar(char data)
{
    while ((UART7_FR_R & 0x0020) != 0)
        ; // Wait until TXFE is 0 (data is available)
    UART7_DR_R = data;
}

void UART7_OutStr(char *str)
{
    while (*str != '\0')
    {
        UART7_OutChar(*str++);
    }
}

void UART7_GetLine(char *buf, int n)
{
    int i = 0;
    while (i < n - 1)
    {
        buf[i] = UART7_InChar();
        if (buf[i] == '\n')
        {
            break;
        }
        i++;
    }
    buf[i] = '\0';
}
