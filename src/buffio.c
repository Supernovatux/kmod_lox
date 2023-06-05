#include "buffio.h"
#include "common.h"
#include "linux/types.h"
#include <linux/printk.h>
#include <linux/slab.h>
#include <linux/stdarg.h>

int initBuff(buffio *b, char *buffer, size_t *size)
{
	FUNBEGIN;
	b->buffer =
		buffer ? buffer :
			 kcalloc(BUFF_DEFAULT_SIZE, sizeof(char), GFP_KERNEL);
	b->size = buffer ? *size : BUFF_DEFAULT_SIZE;
	b->cursor = 0;
	return b->buffer == NULL;
}
int writeRaw(buffio *b, char *s, size_t len, loff_t *offset)
{
	if (*offset + len > b->size) {
		b->size = *offset + len;
		b->buffer = krealloc(b->buffer, b->size, GFP_KERNEL);
		if (b->buffer == NULL) {
			return -1;
		}
	}
	for (size_t i = 0; i < len; i++) {
		b->buffer[*offset + i] = s[i];
	}
	*offset += len;
	return len;
}
int write(buffio *b, char *s, size_t len)
{
	return writeRaw(b, s, len, &b->cursor);
}
int printBuff(buffio *b, char *s, ...)
{
	va_list args;
	va_start(args, s);
	int len = vsnprintf(NULL, 0, s, args);
	va_end(args);
	char *buff = kmalloc(sizeof(char) * (len + 1), GFP_KERNEL);
	va_start(args, s);
	vsprintf(buff, s, args);
	va_end(args);
	int ret = write(b, buff, len);
	kfree(buff);
	return ret;
}
int readBuff(buffio *b, char *s, size_t len, loff_t *offset)
{
	if (*offset + len > b->size) {
		return -1;
	}
	for (size_t i = 0; i < len; i++) {
		s[i] = b->buffer[*offset + i];
	}
	*offset += len;
	return len;
}
void inline freeBuff(buffio *b)
{
	kfree(b->buffer);
}
