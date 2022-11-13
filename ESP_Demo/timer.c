/*
 * timer.c
 *
 *  Created on: Feb 13, 2017
 *      Author: srazak
 *  Modified on: August 25, 2021
 *      Author: efeoflus
 *
 */

#include <stdint.h>
#include <stdbool.h>
#include "15348.h"

void SysTick_Init()
{
    NVIC_ST_CTRL_R = 0;
    NVIC_ST_RELOAD_R = 0x00FFFFFF;
    NVIC_ST_CURRENT_R = 0;
    NVIC_ST_CTRL_R = 0x00000005;

}

/** busy waiting for one tick, 12.5ns if using 80MHZ **/
void SysTick_Wait(uint32_t delay)
{

    NVIC_ST_RELOAD_R = delay - 1;
    NVIC_ST_CURRENT_R = 0;
    while ((NVIC_ST_CTRL_R & 0x00010000) == 0)
    {
    }
}

void SysTick_Wait10ms(uint32_t delay)
{
    uint32_t i;
    for (i = 0; i < delay; i++)
        SysTick_Wait(800000);
}

void SysTick_Wait1ms(uint32_t delay)
{
    uint32_t i;
    for (i = 0; i < delay; i++)
        SysTick_Wait(80000);
}

void SysTick_Wait100microsec(uint32_t delay)
{
    uint32_t i;
    for (i = 0; i < delay; i++)
        SysTick_Wait(8000);
}

void SysTick_Wait10microsec(uint32_t delay)
{
    uint32_t i;
    for (i = 0; i < delay; i++)
        SysTick_Wait(800);
}

void SysTick_Wait1microsec(uint32_t delay)
{
    uint32_t i;
    for (i = 0; i < delay; i++)
        SysTick_Wait(80);
}

