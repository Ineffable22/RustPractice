#include "sum.h"
#include <stdio.h>

int32_t main(void)
{
	int32_t a = 1;
	int32_t b = 2;
	int32_t c = sum(a, b);
	printf("%d + %d = %d\n", a, b, c);
	return 0;
}
