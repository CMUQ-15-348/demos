/*
 * circular_queue.h
 *
 *  Created on: Oct 9, 2022
 *      Author: ryan
 */

#ifndef CIRCULAR_QUEUE_H_
#define CIRCULAR_QUEUE_H_

#define QUEUE_SIZE 8
struct circular_queue
{
    char *data[QUEUE_SIZE];
    int8_t write_to;
    int8_t read_from;
    int8_t num_items;
};

void init_queue(struct circular_queue *q);
void enqueue(struct circular_queue *q, char *cmd);
char* dequeue(struct circular_queue *q);

#endif /* CIRCULAR_QUEUE_H_ */
