#include "15348.h"

// Subroutine to initialize port F pins for input and output
// PF4 and PF0 are input SW1 and SW2 respectively
// PF3,PF2,PF1 are outputs to the LED
void PortF_Init(void)
{
    volatile unsigned long delay;
    SYSCTL_RCGC2_R |= 0x00000020;     // 1) F clock
    delay = SYSCTL_RCGC2_R; // reading register adds a delay, which we might need
    GPIO_PORTF_LOCK_R = 0x4C4F434B;   // 2) unlock PortF PF0
    GPIO_PORTF_CR_R = 0x1F;           // allow changes to PF4-0
    GPIO_PORTF_AMSEL_R = 0x00;        // 3) disable analog function
    GPIO_PORTF_PCTL_R = 0x00000000;   // 4) GPIO clear bit PCTL
    GPIO_PORTF_DIR_R = 0x0E;          // 5) PF4,PF0 input, PF3,PF2,PF1 output
    GPIO_PORTF_AFSEL_R = 0x00;        // 6) no alternate function
    GPIO_PORTF_PUR_R = 0x11;          // 7) enable pullup resistors on PF4,PF0
    GPIO_PORTF_DEN_R = 0x1F;          // 8) enable digital pins PF4-PF0
}

void Delay(void)
{
    unsigned long volatile time;
    time = 727240 * 200 / 91;  // 0.1sec
    while (time)
    {
        time--;
    }
}

unsigned long SW1;

/**
 * main.c
 */
int main(void)
{
    PortF_Init();

    while (1)
    {
        SW1 = GPIO_PORTF_DATA_R & 0x01;
        if (SW1 == 0x00) {
            GPIO_PORTF_DATA_R = 0x08; // Green is on
            Delay();
            GPIO_PORTF_DATA_R = 0x04; // Blue is on
            Delay();
            GPIO_PORTF_DATA_R = 0x02; // Red is on
            Delay();
        }
    }

    return 0;
}
