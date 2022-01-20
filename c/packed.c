#include <stdio.h>

struct A {
	char c;
	unsigned short u;
};

struct B {
	int b;
	char c;
	int d;
};

int main() {
	struct A a;
	a.c = 'c';
	a.u = 15;

	printf("the sizeof A:a is (%d:%d)\n", sizeof(struct A), sizeof(a));

	struct A *pa = &a;
	void *p = (void*)pa;
	char *pc = (char*)pa;
	printf("%c\n", ((char*)p)[0]);
	p+=1;
	printf("%d\n", *((unsigned short*)p));
	p+=1;
	printf("%d\n", *((unsigned short*)p));


	for (int x=0; x < sizeof(a); x++) {
		printf("%x ", *(pc+x));
	}
	printf("\n");

	return 0;
}


