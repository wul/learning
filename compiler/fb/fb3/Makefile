.PHONY: all
all: fb3-1
fb3-1: fb3-1.l fb3-1.y fb3-1funcs.c
	bison -d fb3-1.y
	flex fb3-1.lex
	gcc -o $@ lex.yy.c fb3-1.tab.c fb3-1funcs.c

fb3-2: fb3-2.l fb3-2.y fb3-2funcs.c
	bison -d fb3-2.y
	flex fb3-2.lex
	gcc -o $@ lex.yy.c fb3-2.tab.c fb3-2funcs.c

