#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct MyString {
  const char *hash_code;
  float hash_time;
} MyString;

const struct MyString *Blake3C(int8_t threads_num, const char *s);
