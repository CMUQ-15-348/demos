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
    GPIO_PORTF_CR_R = 0x1F;           // 3) allow changes to PF4-0
    GPIO_PORTF_AMSEL_R = 0x00;        // 4) disable analog function
    GPIO_PORTF_AFSEL_R = 0x00;        // 5) no alternate function
    GPIO_PORTF_PCTL_R = 0x00000000;   // 6) GPIO clear bit PCTL
    GPIO_PORTF_PUR_R = 0x11;          // 7) enable pullup resistors on PF4,PF0
    GPIO_PORTF_DEN_R = 0x1F;          // 8) enable digital pins PF4-PF0
    GPIO_PORTF_DIR_R = 0x0E;          // 9) PF4,PF0 input, PF3,PF2,PF1 output
}

/********* Standard Method of Port Access *********/

//   Global Variables
unsigned long PORTFVAL;

int main_standard(void)
{
    PortF_Init();                 // Call initialization of port PF0-PF4

    while (1)
    {
        PORTFVAL = GPIO_PORTF_DATA_R;
        if ((PORTFVAL & 0x01) == 0)
        {
            GPIO_PORTF_DATA_R = 0x04;     // LED is blue
        }
        else if ((PORTFVAL & 0x10) == 0)
        {
            GPIO_PORTF_DATA_R = 0x02;     // LED is red
        }
        else
        {
            GPIO_PORTF_DATA_R = 0x08;     // LED is green
        }
    }
}

/********* Bit Banding Method of Port Access *********/

// Note: GPIO_PORTF_DATA_R is 0x400253FC
#define BB_PF0       (*((volatile unsigned long *)0x424a7f80))
#define BB_PF4       (*((volatile unsigned long *)0x424a7f90))
#define BB_PF1       (*((volatile unsigned long *)0x424a7f84))
#define BB_PF2       (*((volatile unsigned long *)0x424a7f88))
#define BB_PF3       (*((volatile unsigned long *)0x424a7f8c))
int main_bb(void)
{
    PortF_Init();                 // Call initialization of port PF0-PF4

    while (1)
    {
        // Note: This is additive, because things aren't turned off
        if (BB_PF0 == 0)
        {
            BB_PF2 = 1;     // Blue
        }
        else if (BB_PF4 == 0)
        {
            BB_PF1 = 1;     // Red
        }
        else
        {
            BB_PF3 = 1;     // Green
        }
    }
}

/********* Alternative Method of Port Access *********/
// Note: GPIO_PORTF_DATA_BITS_R is 0x40025000
#define PF0       (*((volatile unsigned long *)0x40025004))
#define PF4       (*((volatile unsigned long *)0x40025040))
#define PF1       (*((volatile unsigned long *)0x40025008))
#define PF2       (*((volatile unsigned long *)0x40025010))
#define PF3       (*((volatile unsigned long *)0x40025020))
int main_alt_bb(void)
{
    PortF_Init();                 // Call initialization of port PF0-PF4

    while (1)
    {
        // Note: This is additive, because things aren't turned off
        if (PF0 == 0)
        {
            PF2 = 0x04;     // Blue
        }
        else if (PF4 == 0)
        {
            PF1 = 0x02;     // Red
        }
        else
        {
            PF3 = 0x08;     // Turn on green
        }
    }
}


int main(void)
{
    main_standard();
}
