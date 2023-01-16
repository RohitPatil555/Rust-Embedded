#include <stdio.h>
#include <stdint.h>

// Rust proto
void rust_hello(void);
uint32_t rust_add(uint32_t a, uint32_t b);
void rust_array(uint32_t *p, uint32_t count);

int main()
{
	rust_hello();

	// call rust add api
	uint32_t c = rust_add( 2, 3);
	printf("2 + 3 = %d \n", c);

	// passing array to rust function.
	uint32_t arr[5] = { 0x11, 0x22, 0x33, 0x44, 0x55 };
	rust_array( arr, 5);

	// passing structure to rust function.

	// passing enum to rust function.

	return 0;
}
