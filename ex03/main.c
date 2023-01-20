#include <stdio.h>
#include <stdint.h>
#include <string.h>

typedef struct {
	uint32_t a;
	uint32_t b;
} myinfo_t;

typedef enum {
	HELLO = 1,
	BYE
} myenum_t;

// Rust proto
void rust_hello(void);
uint32_t rust_add(uint32_t a, uint32_t b);
void rust_array(uint32_t *p, uint32_t count);
void rust_string(char *p, uint32_t count);
void rust_struct_print(myinfo_t * ptr);
void rust_enum_print(myenum_t val);

int main()
{
	rust_hello();

	// call rust add api
	uint32_t c = rust_add( 2, 3);
	printf("2 + 3 = %d \n", c);

	// passing array to rust function.
	uint32_t arr[5] = { 0x11, 0x22, 0x33, 0x44, 0x55 };
	rust_array( arr, 5);

	// passing string to rust
	char * s = "Yes, got me";
	rust_string( s, strlen(s));

	// passing structure to rust function.
	myinfo_t m = { 0x66, 0x77 };
	rust_struct_print( &m );	

	// passing enum to rust function.
	myenum_t v1 = HELLO;
	rust_enum_print(v1);

	v1 = BYE;
	rust_enum_print(v1);

	return 0;
}
