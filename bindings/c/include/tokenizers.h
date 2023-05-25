#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Model
 */
typedef struct Model Model;

extern const uint32_t UNKNOWN_TOKEN_ID;

void string_destroy(char *c_string);

struct Model *wordlevel_create(const char *vocab_filename, const char *unk_token);

void wordlevel_destroy(struct Model *model);

uint32_t token_to_id(struct Model *model, const char *token);
