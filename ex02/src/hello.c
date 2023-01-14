#include <stdio.h>

// generic type handling
int hello_to(int x)
{
	printf("Hello %d !!! \n", x);
	return 555;
}

// string handling
void hello_name(const char * name)
{
	printf("Hello %s .. \n", name);
}

// array handling.
void print_array(int *p, int count)
{
	int i = 0;

	for(i =0; i< count; i++)
	{
		printf("Array Element at %d => %d \n", i, p[i]);
	}
}


// enum handling.
typedef enum {
	test0,
	test1,
	test88 = 88
} test_e;

void print_enum(test_e val)
{
	switch(val)
	{
	case test0:
		printf("test case 0 \n");
		break;
	case test1:
		printf("test case 1 \n");
		break;
	case test88:
		printf("test case 88 \n");
		break;
	default:
		printf("Unhandled enum \n");
	}

	printf("Enum val : %d \n", (int)val);
}

// string handling.
typedef struct {
	int a;
	int b;
} test_t;

void print_struct(test_t * p)
{
	printf("Struct test_t a : %d \n", p->a);
	printf("Struct test_t b : %d \n", p->b);
}



