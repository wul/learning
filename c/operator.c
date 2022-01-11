#include <stdio.h>

/*
运算符  &i      |       ^         ~     <<     >>
说明    按位与	按位或	按位异或  取反  左移   右移
*/

int getbit(int num, int pos)
{
	if (num & (1 << pos)) {
		return 1;
	} else {
		return 0;
	}
}

void print_binary(int num) 
{
        printf("Binary output for %d:", num);
	for (int k = 31; k >=0; k--) {
		printf("%d", getbit(num, k));
	}
	printf("\n");
}
	
int main() {
	int i = 1;
        int j = -2;
	int n = -1;
	print_binary(i);
	print_binary(j);
	print_binary(n);

	unsigned int k = 1;
	print_binary(k);

        printf("Test with integer %d\n", i);
	printf("&0: %d\n", i & 0);
	printf("|0: %d\n", i | 0);
	printf("^1: %d\n", i ^ 1);
	printf("i=%d, ~i: %d, ~1=%d\n", i, ~i, ~1);
	printf("i<<1: %d\n", i<<1);
	printf("i>>1: %d\n", i>>1);
	return 0;
}
