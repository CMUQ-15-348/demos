// i2c0.c
// Runs on LM4F120/TM4C123
// Provide a function that initializes, sends, and receives the I2C0 module
// interfaced with an HMC6352 compass or TMP102 thermometer.
// Daniel Valvano
// July 2, 2014

// Re-organized by Ryan Riley for the course 15-348
// September, 2022

/* This example accompanies the book
 "Embedded Systems: Real Time Interfacing to Arm Cortex M Microcontrollers",
 ISBN: 978-1463590154, Jonathan Valvano, copyright (c) 2013
 Section 8.6.4 Programs 8.5, 8.6 and 8.7

 Copyright 2013 by Jonathan W. Valvano, valvano@mail.utexas.edu
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
#include <stdint.h>
#include "15348.h"
#include "serial.h"

#define I2C_MCS_ACK             0x00000008  // Data Acknowledge Enable
#define I2C_MCS_DATACK          0x00000008  // Acknowledge Data
#define I2C_MCS_ADRACK          0x00000004  // Acknowledge Address
#define I2C_MCS_STOP            0x00000004  // Generate STOP
#define I2C_MCS_START           0x00000002  // Generate START
#define I2C_MCS_ERROR           0x00000002  // Error
#define I2C_MCS_RUN             0x00000001  // I2C Master Enable
#define I2C_MCS_BUSY            0x00000001  // I2C Busy
#define I2C_MCR_MFE             0x00000010  // I2C Master Function Enable
#define MAXRETRIES              5           // number of receive attempts before giving up

void I2C_Init(void)
{
    SYSCTL_RCGCI2C_R |= 0x01;   // Activate I2C0
    SYSCTL_RCGCGPIO_R |= 0x02;  // Activate port B
    while ((SYSCTL_PRGPIO_R & 0x02) == 0); // Wait for port to activate
    GPIO_PORTB_AFSEL_R |= 0x0C; // enable alt func on PB2, PB3
    GPIO_PORTB_ODR_R |= 0x08;   // open drain on PB3 (sda)
    GPIO_PORTB_PCTL_R = (GPIO_PORTB_PCTL_R & 0xFFFF00FF) + 0x00003300; // I2C as the alternate functionality
    GPIO_PORTB_DEN_R |= 0x0C; // Enable digital IO for PB2, PB3
    I2C0_MCR_R = 0x10; // Master mode
    I2C0_MTPR_R = 0x27; // Choose the baud rate, 100 KHz.  From table 16-2 in the data sheet.
}

uint32_t I2C_Send1(int8_t slave, uint8_t data1)
{
    while (I2C0_MCS_R & I2C_MCS_BUSY); // wait for I2C ready
    I2C0_MSA_R = (slave << 1) & 0xFE;    // MSA[7:1] is slave address
    I2C0_MSA_R &= ~0x01;             // MSA[0] is 0 for send
    I2C0_MDR_R = data1 & 0xFF;         // prepare first byte
    I2C0_MCS_R = (0
    //   & ~I2C_MCS_ACK     // no data ack (no data on send)
            | I2C_MCS_STOP     // generate stop
            | I2C_MCS_START    // generate start/restart
            | I2C_MCS_RUN);    // master enable
    while (I2C0_MCS_R & I2C_MCS_BUSY);    // wait for transmission done
          // return error bits
    if (I2C0_MCS_R & (I2C_MCS_DATACK | I2C_MCS_ADRACK | I2C_MCS_ERROR))
    {
        SerialWrite("Error: ");
        SerialWriteHex(
                (I2C0_MCS_R & (I2C_MCS_DATACK | I2C_MCS_ADRACK | I2C_MCS_ERROR)));
    }
    return (I2C0_MCS_R & (I2C_MCS_DATACK | I2C_MCS_ADRACK | I2C_MCS_ERROR));
}

uint32_t I2C_Send2(int8_t slave, uint8_t data1, uint8_t data2)
{
    while (I2C0_MCS_R & I2C_MCS_BUSY);  // wait for I2C ready
    I2C0_MSA_R = (slave << 1) & 0xFE;    // MSA[7:1] is slave address
    I2C0_MSA_R &= ~0x01;             // MSA[0] is 0 for send
    I2C0_MDR_R = data1 & 0xFF;         // prepare first byte
    I2C0_MCS_R = (0
    //  & ~I2C_MCS_ACK     // no data ack (no data on send)
    //   & ~I2C_MCS_STOP    // no stop
            | I2C_MCS_START    // generate start/restart
            | I2C_MCS_RUN);    // master enable
    while (I2C0_MCS_R & I2C_MCS_BUSY);    // wait for transmission done
          // check error bits
    if ((I2C0_MCS_R & (I2C_MCS_DATACK | I2C_MCS_ADRACK | I2C_MCS_ERROR)) != 0)
    {
        I2C0_MCS_R = (0                // send stop if nonzero
        //  & ~I2C_MCS_ACK     // no data ack (no data on send)
                | I2C_MCS_STOP     // stop
        //  & ~I2C_MCS_START   // no start/restart
        //  & ~I2C_MCS_RUN    // master disable
                );
        // return error bits if nonzero
        return (I2C0_MCS_R & (I2C_MCS_DATACK | I2C_MCS_ADRACK | I2C_MCS_ERROR));
    }
    I2C0_MDR_R = data2 & 0xFF;         // prepare second byte
    I2C0_MCS_R = (0
    // & ~I2C_MCS_ACK     // no data ack (no data on send)
            | I2C_MCS_STOP     // generate stop
            // & ~I2C_MCS_START   // no start/restart
            | I2C_MCS_RUN);    // master enable
    while (I2C0_MCS_R & I2C_MCS_BUSY);    // wait for transmission done
          // return error bits
    if (I2C0_MCS_R & (I2C_MCS_DATACK | I2C_MCS_ADRACK | I2C_MCS_ERROR))
    {
        SerialWrite("Error: ");
        SerialWriteHex(
                (I2C0_MCS_R & (I2C_MCS_DATACK | I2C_MCS_ADRACK | I2C_MCS_ERROR)));
    }
    return (I2C0_MCS_R & (I2C_MCS_DATACK | I2C_MCS_ADRACK | I2C_MCS_ERROR));
}

uint8_t I2C_Recv(int8_t slave)
{
    int retryCounter = 1;
    do
    {
        while (I2C0_MCS_R & I2C_MCS_BUSY);   // wait for I2C ready
        I2C0_MSA_R = (slave << 1) & 0xFE;    // MSA[7:1] is slave address
        I2C0_MSA_R |= 0x01;              // MSA[0] is 1 for receive
        I2C0_MCS_R = (0
        // & ~I2C_MCS_ACK     // negative data ack (last byte)
                | I2C_MCS_STOP     // generate stop
                | I2C_MCS_START    // generate start/restart
                | I2C_MCS_RUN);    // master enable
        while (I2C0_MCS_R & I2C_MCS_BUSY);    // wait for transmission done
        retryCounter = retryCounter + 1;        // increment retry counter
    }                                         // repeat if error
    while (((I2C0_MCS_R & (I2C_MCS_ADRACK | I2C_MCS_ERROR)) != 0)
            && (retryCounter <= MAXRETRIES));
    return (I2C0_MDR_R & 0xFF);          // usually returns 0xFF on error
}

uint16_t I2C_Recv2(int8_t slave)
{
    uint8_t data1, data2;
    int retryCounter = 1;
    do
    {
        while (I2C0_MCS_R & I2C_MCS_BUSY);  // wait for I2C ready
        I2C0_MSA_R = (slave << 1) & 0xFE;    // MSA[7:1] is slave address
        I2C0_MSA_R |= 0x01;              // MSA[0] is 1 for receive
        I2C0_MCS_R = (0 | I2C_MCS_ACK      // positive data ack
                //  & ~I2C_MCS_STOP    // no stop
                | I2C_MCS_START    // generate start/restart
                | I2C_MCS_RUN);    // master enable
        while (I2C0_MCS_R & I2C_MCS_BUSY);    // wait for transmission done
        data1 = (I2C0_MDR_R & 0xFF);       // MSB data sent first
        I2C0_MCS_R = (0
        //  & ~I2C_MCS_ACK     // negative data ack (last byte)
                | I2C_MCS_STOP     // generate stop
                //  & ~I2C_MCS_START   // no start/restart
                | I2C_MCS_RUN);    // master enable
        while (I2C0_MCS_R & I2C_MCS_BUSY);    // wait for transmission done
        data2 = (I2C0_MDR_R & 0xFF);       // LSB data sent last
        retryCounter = retryCounter + 1;        // increment retry counter
    }                                         // repeat if error
    while (((I2C0_MCS_R & (I2C_MCS_ADRACK | I2C_MCS_ERROR)) != 0)
            && (retryCounter <= MAXRETRIES));
    // data2 is MSB, data1 is LSB in this example.
    return (data2 << 8) + data1;              // usually returns 0xFFFF on error
}
