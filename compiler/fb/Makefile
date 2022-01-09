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
