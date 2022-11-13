/*
 * uart.h
 *
 *  Created on: Oct 5, 2022
 *      Author: ryan
 */

#ifndef UART_H_
#define UART_H_

void InitUART7();
char UART7_InChar(void);
void UART7_OutChar(char data);
void UART7_OutStr(char *str);
void UART7_GetLine(char *buf, int n);

#endif /* UART_H_ */
