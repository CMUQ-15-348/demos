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
}

unsigned long timer_val = 0;

void SysTick_Handler(void)
{
    timer_val++;
}

// We have four states
#define WAITING 0x00
#define PRESSED 0x01
#define DEBOUNCE_RELEASE  0x02
#define RELEASED 0x03

// The input is 1 bit: The current switch value

struct State
{
    unsigned char next[2]; // Next state based on 1-bit input (switch value)
    unsigned char out;     // Whether or not we "output"/take action in this take.  (0 or non-zero)
    unsigned int time;     // min time to stay in this state (intervals of 1 ms)
};

struct State FSM[4] = {
        { { WAITING, PRESSED }, 0x00, 0 }, // State WAITING
        { { DEBOUNCE_RELEASE, PRESSED }, 0x00, 7 }, // State PRESSED
        { { RELEASED, RELEASED }, 0x00, 3 }, // State DEBOUNCE_RELEASE
        { { WAITING, PRESSED }, 0x01, 0 }  // State RELEASED
};

unsigned char curState = WAITING;

void SysTickInterruptInit()
{
    NVIC_ST_CTRL_R = 0;
    NVIC_ST_RELOAD_R = 80000; // exactly 1ms
    NVIC_ST_CURRENT_R = 0;
    NVIC_ST_CTRL_R = 0x00000007;
}

// This function does all the processing for the SW1 state machine.
// It needs to be called from the main function
// It returns 0 when the button has not been pressed and released, and 1 otherwise.
int check_sw1(void)
{
    // Check if we've been in the state long enough to move on if needed
    if (timer_val >= FSM[curState].time) {
        // Check SW1 status
        int inp = (GPIO_PORTF_DATA_R & 0x10)>>4; // Shift right 4 bits so our switch status is the least significant bit
        inp ^= 0x01; // Invert the switch status because my switch is active low, but my state machine assumes active high

        // Choose the next state
        if (curState != FSM[curState].next[inp]) {
            // If we are going to change states, then reset the counter
            timer_val = 0;
        }
        curState = FSM[curState].next[inp];
    }
    return FSM[curState].out;
}

/**
 * main.c
 */
int main(void)
{
    PLLInit();
    PortF_Init();
    SysTickInterruptInit();

    while(1) {
        int status = check_sw1();

        // If the switch has been pushed, toggle the led
        if (status) {
            GPIO_PORTF_DATA_R ^= 0x04;
        }

        // Note: Nothing in this loop ever waits, so pushing and holding the button doesn't cause any problems.

    }
}
