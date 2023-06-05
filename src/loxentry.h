#pragma once
#ifndef LOXENTRY_H
#define LOXENTRY_H
#include "kmain.h"
#include "linux/types.h"
int threadedLox(void *data);
bool lox_data_free(lox_data_t **lox_data);
#endif
