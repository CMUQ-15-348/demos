#include <string.h>
#include <stdlib.h>
#include "15348.h"
#include "serial.h"
#include "uart.h"
#include "timer.h"
#include "circular_queue.h"

void process_line(char *line);

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

/*
 * A circular queue for commands to send to the ESP
 * During the main loop, we'll pull them off and process them
 * with a state machine
 */
struct circular_queue cmd_q;

// Some global variables for my state
char line[128];
uint16_t line_c;
uint8_t got_ok = 0;

void UART7_handler(void)
{
    // Interrupt because there is data in the receive FIFO
    if ((UART7_RIS_R & 0x10) != 0)
    {
        UART7_ICR_R |= 0x10; // Acknowledge the interrupt
    }

    // Receiver timeout.  There is data in the FIFO, and we haven't received any more for 32 UART cycles.
    // This interrupt is needed because sometimes there is data that isn't handled by a previous interrupt,
    // and this makes sure we clear out the entire FIFO.
    if ((UART7_RIS_R & 0x40) != 0)
    {
        UART7_ICR_R |= 0x40; // Acknowledge the interrupt
    }

    while ((UART7_FR_R & 0x0010) == 0)
    {
        char c = (char) (UART7_DR_R & 0xFF);
        line[line_c++] = c;
        if (c == '\n')
        {
            line[line_c] = '\0';
            process_line(line);
            line_c = 0;
        }
    }
}

void process_line(char *line)
{
    SerialWrite(line);
    if (strcmp(line, "OK\r\n") == 0)
    {
        got_ok = 1;
    }
}

void esp_command(char *cmd)
{
    got_ok = 0;
    UART7_OutStr(cmd);
}

/*
 * State machine stuff for sending commands and waiting for an OK
 * FREE means we aren't currently doing anything and are available to send a process a command
 * SENT_COMMAND means we've sent a command and are waiting to get back "OK\r\n"
 */
#define FREE 0
#define SENT_COMMAND 1
uint8_t cmd_state = FREE;

char buf[128];
/**
 * main.c
 */
int main(void)
{
    //char c[2] = {0,0};
    PLLInit();
    SysTick_Init();
    SetupSerial();
    InitUART7();

    SerialWriteLine("-----Go ESP-----");

    init_queue(&cmd_q);

    // The initial commands that I want to see run
    enqueue(&cmd_q, "AT\r\n");
    enqueue(&cmd_q, "AT+CWMODE=1\r\n");
    enqueue(&cmd_q, "AT+CWJAP=\"cs4qatar\",\"csevent16\"\r\n");
    enqueue(&cmd_q, "AT+CIFSR\r\n");
    enqueue(&cmd_q, "AT+CIPMUX=1\r\n");
    enqueue(&cmd_q, "AT+CIPSERVER=1,1234\r\n");

    while (1)
    {
        // A basic state machine to know what we are doing with regards to commands.
        if (cmd_state == FREE && cmd_q.num_items > 0)
        {
            char *cmd = dequeue(&cmd_q);
            esp_command(cmd);
            cmd_state = SENT_COMMAND;
        }
        else if (cmd_state == SENT_COMMAND && got_ok == 1)
        {
            got_ok = 0;
            cmd_state = FREE;
        }
    }
}
