#include "15348.h"

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

    GPIO_PORTF_IS_R &= ~0x01; // PF0 edge-sensitive
    GPIO_PORTF_IBE_R &= ~0x01; // PF0 shouldn't trigger on both edges
    GPIO_PORTF_IEV_R |= 0x01; // PF0 is rising edge sensitive
    GPIO_PORTF_ICR_R |= 0x01; // Clear any interrupts for PF0
    GPIO_PORTF_IM_R |= 0x01; // Setup PF0 for interrupts
    NVIC_PRI7_R = (NVIC_PRI7_R&0xFF00FFFF) | 0x00A00000; // Priority 5
    NVIC_EN0_R = (1 << 30); // Enable interrupts
}

unsigned long count = 0;

void GPIOPortF_Handler(void)
{
    GPIO_PORTF_ICR_R = 0x01; // acknowledge the interrupt
    count++;
}

/**
 * main.c
 */
int main(void)
{
	PLLInit();
	PortF_Init();

	GPIO_PORTF_DATA_R = 0x02;

	while(1) {
	    if (count >= 10) {
	        GPIO_PORTF_DATA_R = 0x04;
	    }

	}
}
