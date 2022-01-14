.PHONY: all
all: fb2-3 fb2-4 fb2-5

fb2-3: fb2-3.l
	flex fb2-3.l
	gcc -o $@ lex.yy.c -ll

fb2-4: fb2-4.l
	flex fb2-4.l
	gcc -o $@ lex.yy.c -ll

fb2-5: fb2-5.lex
	flex $<
	gcc -o $@ lex.yy.c -ll

