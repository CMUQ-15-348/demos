/*
 * serial.h
 *
 *  Created on: Feb 12, 2017
 *      Author: srazak
 */

#ifndef SERIAL_H_
#define SERIAL_H_

#include <stdint.h>

void SetupSerial();
void SerialWrite(char*);
void SerialWriteInt(int);
void SerialWriteLine(char*);
void SerialWriteHex(unsigned int n);
void SerialWriteShortHex(uint32_t n);
#endif /* SERIAL_H_ */
