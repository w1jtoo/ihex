#include <stdio.h>
#include <sys/mman.h>

int testing(unsigned char* code, size_t code_size) {
	void* mem  = mmap(NULL, code_size, PROT_WRITE | PROT_EXEC, MAP_PRIVATE, -1, 0);
	
	memcpy(mem, code, code_size);

	int (*func) () = mem;

	func(7, 5);
} 