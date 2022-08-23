#include <stdint.h>
#include "15348.h"
#include "timer.h"
#include "debounce.h"

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

#define ZERO 0
#define HALF 1
#define FULL 2
#define DONE 3

struct State {
    unsigned char next[4];
    unsigned char out;
    unsigned int time;  // The minimum amount of time you must stay in this state before transitioning
};

struct State FSM[4] = {
                       { {ZERO, HALF, FULL, ZERO}, 0x02, 0}, // State 0.00
                       { {HALF, FULL, DONE, ZERO}, 0x02, 0}, // State 0.50
                       { {FULL, DONE, DONE, ZERO}, 0x02, 0}, // State 1.00
                       { {ZERO, HALF, FULL, ZERO}, 0x08, 100}  // State >=1.50
};

unsigned char curState = ZERO;

int main(void)
{
	PLLInit();
	SysTick_Init();
	PortF_Init();

	while(1) {
	    // Set output based on current state
	    GPIO_PORTF_DATA_R = FSM[curState].out;

	    // Wait the min time
	    if (FSM[curState].time)
	        SysTick_Wait10ms(FSM[curState].time);

	    // Read the input
	    unsigned char inp = 0;
	    if (get_sw1()) {
	        inp |= 0x01;
	    }

	    if (get_sw2()) {
	        inp |= 0x02;
	    }

	    // Pick the next state
	    curState = FSM[curState].next[inp];

	}
}
