#include <stdio.h>
#include <sys/mman.h>

int i4_write_memory(unsigned char* code, size_t code_size) {
	void* mem  = mmap(NULL, code_size, PROT_WRITE | PROT_EXEC, MAP_PRIVATE, -1, 0); // declareting mem as executable
	memcpy(mem, code, code_size); // copy value
	int (*func) () = mem; // cast to functions 
	func(7, 5);

	return 1; 
}
