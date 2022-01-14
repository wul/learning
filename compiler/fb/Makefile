.PHONY: all
all: fb1-3 fb1-4 fb1-5 fb2-3 fb2-4 fb2-5 fb3-1
fb1-5: fb1-5.l fb1-5.y
	bison -d fb1-5.y
	flex fb1-5.l
	gcc -o $@ fb1-5.tab.c lex.yy.c -ll
fb1-4: fb1-4.l
	flex fb1-4.l
	gcc -o $@ lex.yy.c -ll
fb1-3: fb1-3.l
	flex fb1-3.l
	gcc -o $@ lex.yy.c -ll
count: count.l
	flex count.l
	gcc -o $@ lex.yy.c -ll

count2: count2.l
	flex count2.l
	gcc -o $@ lex.yy.c -ll
fb2-3: fb2-3.l
	flex fb2-3.l
	gcc -o $@ lex.yy.c -ll

fb2-4: fb2-4.l
	flex fb2-4.l
	gcc -o $@ lex.yy.c -ll

fb2-5: fb2-5.lex
	flex $<
	gcc -o $@ lex.yy.c -ll

fb3-1: fb3-1.lex fb3-1.y fb3-1funcs.c
	bison -d fb3-1.y
	flex fb3-1.lex
	gcc -o $@ lex.yy.c fb3-1.tab.c fb3-1funcs.c
