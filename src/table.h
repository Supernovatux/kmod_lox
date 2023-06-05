#ifndef clox_table_h
#define clox_table_h
#include "common.h"
#include "value.h"
#include <linux/types.h>
typedef struct {
	ObjString *key;
	Value value;
} Entry;
typedef struct {
	int count;
	int capacity;
	Entry *entries;
} Table;
void initTable(Table *table);
void freeTable(Table *table);
bool tableSet(Table *table, ObjString *key, Value value);
void tableAddAll(Table *from, Table *to);
bool tableGet(Table *table, ObjString *key, Value *value);
bool tableDelete(Table *table, ObjString *key);
ObjString *tableFindString(Table *table, const char *chars, int length,
			   uint32_t hash);
void markTable(Table *table);

void tableRemoveWhite(Table *table);
#endif
