/*
 * compass.c
 *
 *  Created on: Sep 29, 2022
 *      Author: ryan
 */
#include <stdint.h>
#include "i2c0.h"
#include "compass.h"

// I2C address of the compass
#define ADDRESS 0x0D

// Register addresses
#define REG_XOUT_LSB 0x00
#define REG_STATUS 0x06
#define REG_CR1 0x09
#define REG_CR2 0x0A
#define REG_SR_PERIOD 0x0B

void Compass_Init(void)
{
    // Set/Reset Period to 0x01.  Why?  The datasheet says so.
    I2C_Send2(ADDRESS, REG_SR_PERIOD, 0x01);
    // OSR of 512, 8 Gauss, 10 Hz sample rate, Continuous sampling
    I2C_Send2(ADDRESS, REG_CR1, 0x11);
    // Enable roll-over for reading data registers and disable DRDY interrupt
    I2C_Send2(ADDRESS, REG_CR2, 0x41);
}

void Compass_Wait_Until_Available(void)
{
    // Wait until data is available to read
    // We check the status bit in the status register
    uint8_t ret;
    do {
        I2C_Send1(ADDRESS, REG_STATUS);
        ret = I2C_Recv(ADDRESS);
    } while ((ret & 0x01) == 0x00);
}

uint16_t Compass_getX()
{
    // Set current register to the first data register
    I2C_Send1(ADDRESS, REG_XOUT_LSB);
    return I2C_Recv2(ADDRESS);
}

uint16_t Compass_getY()
{
    // Set current register to the first data register
    I2C_Send1(ADDRESS, REG_XOUT_LSB+2);
    return I2C_Recv2(ADDRESS);
}

uint16_t Compass_getZ()
{
    // Set current register to the first data register
    I2C_Send1(ADDRESS, REG_XOUT_LSB+4);
    return I2C_Recv2(ADDRESS);
}
