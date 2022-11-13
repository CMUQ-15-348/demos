/*
 * circular_queue.c

 * A circular queue of strings.
 *
 * Note: This doesn't really do any bounds checking, so it will happily let you put too many items in it and overwrite some.
 *
 *  Created on: Oct 9, 2022
 *      Author: ryan
 */
#include <stdint.h>
#include <stdio.h>
#include "circular_queue.h"

void init_queue(struct circular_queue *q)
{
    q->write_to = 0;
    q->read_from = -1;
    q->num_items = 0;
}

void enqueue(struct circular_queue *q, char *cmd)
{
    if (q->num_items == QUEUE_SIZE)
    {
        return;
    }
    // Add to the "end" of the queue
    q->data[q->write_to++] = cmd;
    // Wrap around the counter if needed
    q->write_to = q->write_to % QUEUE_SIZE;
    // Handle the initial case
    if (q->read_from == -1)
    {
        q->read_from = 0;
    }
    q->num_items++;
}

char* dequeue(struct circular_queue *q)
{
    if (q->num_items == 0)
    {
        return NULL;
    }
    char *ret = q->data[q->read_from++];
    q->read_from = q->read_from % QUEUE_SIZE;
    q->num_items--;
    return ret;
}

