#include <stdio.h>
#include <sys/mman.h>
#include <string.h>

const void* i4_write_memory(unsigned char* code, size_t code_size) {
    void* mem  = mmap(NULL,
                4096,
                PROT_READ | PROT_WRITE | PROT_EXEC,
                MAP_PRIVATE | MAP_ANON,
                -1,
                0); // declareting mem as executable

    memcpy(mem, code, code_size); // copy value
    return mem;
}
