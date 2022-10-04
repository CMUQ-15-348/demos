/*
 * screen.h
 *
 *  Created on: Sep 29, 2022
 *      Author: ryan
 */


void initDisplay();
void clearDisplay();
void clearLine(uint8_t row);
void putChar(unsigned char ch);
void putString(const char *string);
void setTextXY(unsigned char row, unsigned char col);
void displayOn();
void displayOff();
uint8_t putNumber(long long_num);
