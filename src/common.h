#ifndef clox_common_h
#define clox_common_h
#include "kmain.h"
#include "linux/kernel.h"
#include <linux/stddef.h>
#include <linux/types.h>
//#define printff(a, ...)                                                        \
//  printk(KERN_ALERT "file:- %s %s(): " a, __FILE__, __func__, ##__VA_ARGS__)
//#define printff(a, ...)                                                        \
//  printfff(KERN_ALERT "file:- %s %s(): " a, __FILE__, __func__, ##__VA_ARGS__)
#define FUNBEGIN pr_debug(" begin")
#define FUNMID(x) pr_debug(" middle %d", x)
#define FUNEND pr_debug(" end")
// #define DEBUG_PRINT_CODE
// #define DEBUG_TRACE_EXECUTION
#define UINT8_COUNT (U8_MAX + 1)
#endif
