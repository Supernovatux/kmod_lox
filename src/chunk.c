#include "chunk.h"
#include "common.h"
#include "memory.h"
#include "vm.h"

void initChunk(Chunk *chunk) {
  chunk->count = 0;
  chunk->capacity = 0;
  chunk->code = NULL;
  /**! TODO: run length encoding
  **@todo run length encoding
  **/
  chunk->lines = NULL;
  initValueArray(&chunk->constants);
}
void writeChunk(Chunk *chunk, uint8_t byte, int line) {
  if (chunk->capacity < chunk->count + 1) {
    int oldCapacity = chunk->capacity;
    chunk->capacity = GROW_CAPACITY(oldCapacity);
    chunk->code =
        GROW_ARRAY(uint8_t, chunk->code, oldCapacity, chunk->capacity);
    chunk->lines = GROW_ARRAY(int, chunk->lines, oldCapacity, chunk->capacity);
  }
  chunk->lines[chunk->count] = line;
  chunk->code[chunk->count] = byte;
  chunk->count++;
}
void freeChunk(Chunk *chunk) {
  FREE_ARRAY(uint8_t, chunk->code, chunk->capacity);
  FREE_ARRAY(int, chunk->lines, chunk->capacity);
  freeValueArray(&chunk->constants);
  initChunk(chunk);
}
int addConstant(Chunk *chunk, Value value) {
  FUNBEGIN;
  push(value);
  FUNMID(1);
  writeValueArray(&chunk->constants, value);
  FUNMID(2);
  pop();
  FUNEND;
  return chunk->constants.count - 1;
}
