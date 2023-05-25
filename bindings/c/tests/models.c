#include <stdio.h>
#include <stdint.h>
#include <assert.h>

#include "tokenizers.h"

int main(int argc, char *argv[]) {
    char * vocab = "data/gpt2-vocab.json";
    char * unk = "[UNK]";
    Model * model = wordlevel_create(vocab, unk);

    uint32_t id;
    id = token_to_id(model, "hello");
    assert(id == 31373);

    id = token_to_id(model, "thistokendoesnotexistinthevocab");
    assert(id == UINT32_MAX);

    return 0;
}
