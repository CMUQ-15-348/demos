// i2c0.h
// Runs on LM4F120/TM4C123
// Provide a function that initializes, sends, and receives the I2C0 module
// interfaced with an HMC6352 compass or TMP102 thermometer.
// Daniel Valvano
// July 2, 2014

// Re-organized by Ryan Riley for the course 15-348
// September, 2022

/* This example accompanies the book
 "Embedded Systems: Real Time Interfacing to Arm Cortex M Microcontrollers",
 ISBN: 978-1463590154, Jonathan Valvano, copyright (c) 2014
 Section 8.6.4 Programs 8.5, 8.6 and 8.7

 Copyright 2014 by Jonathan W. Valvano, valvano@mail.utexas.edu
 You may use, edit, run or distribute this file
 as long as the above copyright notice remains
 THIS SOFTWARE IS PROVIDED "AS IS".  NO WARRANTIES, WHETHER EXPRESS, IMPLIED
 OR STATUTORY, INCLUDING, BUT NOT LIMITED TO, IMPLIED WARRANTIES OF
 MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE APPLY TO THIS SOFTWARE.
 VALVANO SHALL NOT, IN ANY CIRCUMSTANCES, BE LIABLE FOR SPECIAL, INCIDENTAL,
 OR CONSEQUENTIAL DAMAGES, FOR ANY REASON WHATSOEVER.
 For more information about my classes, my research, and my books, see
 http://users.ece.utexas.edu/~valvano/
 */

#ifndef I2C0_H_
#define I2C0_H_
#include <stdint.h>

/*
 *  I2C0SCL connected to PB2
 *  I2C0SDA connected to PB3
 *  SCL and SDA lines should be pulled high by resistors.  Likely 2.2k or 4.7k
 *  (check your data sheet).  For some modules, these resistors are already
 *  on the breakout board.
 */
void I2C_Init(void);

/*
 * Receives one byte from specified slave
 */
uint8_t I2C_Recv(int8_t slave);

/*
 * Receives two bytes from specified slave and returns them as a 16-bit value.
 * The first byte received is the LSB, the second byte received is the MSB.
 */
uint16_t I2C_Recv2(int8_t slave);

/*
 * Sends one byte to specified slave
 * Returns 0 if successful, nonzero if error
 */
uint32_t I2C_Send1(int8_t slave, uint8_t data1);

/*
 * Sends two bytes to specified slave
 * Returns 0 if successful, nonzero if error
 */
uint32_t I2C_Send2(int8_t slave, uint8_t data1, uint8_t data2);

#endif /* I2C0_H_ */
