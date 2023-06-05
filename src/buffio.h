/* A library to print and scan to a char* buffer.
 * Manages the buffer and the cursor position.
 */

#ifndef BUFFIO
#define BUFFIO
#include <linux/types.h>
#define BUFF_DEFAULT_SIZE 0x100
typedef struct {
	char *buffer;
	loff_t cursor;
	size_t size;
} buffio;
extern buffio *lox_input_buffer;
extern buffio *lox_output_buffer;
int initBuff(buffio *b, char *buffer, size_t *size);
int writeRaw(buffio *b, char *s, size_t len, loff_t *offset);
int printBuff(buffio *b, char *s, ...);
int readBuff(buffio *b, char *s, size_t len, loff_t *offset);
void freeBuff(buffio *b);
#endif
