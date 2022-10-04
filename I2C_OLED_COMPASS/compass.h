/*
 * compass.h
 *
 *  Created on: Sep 29, 2022
 *      Author: ryan
 */

#ifndef COMPASS_H_
#define COMPASS_H_

void Compass_Init(void);
void Compass_Wait_Until_Available(void);
uint16_t Compass_getX();
uint16_t Compass_getY();
uint16_t Compass_getZ();

#endif /* COMPASS_H_ */
