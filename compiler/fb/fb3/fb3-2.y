%{
#include <stdio.h>
#include <stdlib.h>
#include "fb3-2.h"
%}

/* 定义了可能拥有的数据类型 */
%union {
    struct ast     *a;
    double          d;
    int             fn; /* 指定函数 */
    struct symbol  *s;  /* 指定符号 */
    struct symlist *sl;
}

/* 声明记号 
 * <val> 标明 token 的类型， 与union中的一致 			
 */		
%token <d>  NUMBER
%token <s>  NAME
%token <fn> FUNC
%token EOL

%token		    IF THEN ELSE WHILE DO LET
			
%nonassoc  <fn>     CMP
%right	   '='
%left	   '+' '-'
%left	   '*' '/'
%nonassoc  '|'      UMINUS

%type <a> exp stmt list explist
%type <sl> symlist
			
%start calclist
%%

stmt: 		IF exp THEN list {$$ = newflow('I', $2, $4, NULL);}
	| 	IF exp THEN list ELSE list {$$ = newflow('I', $2, $4, $6);}
	| 	WHILE exp DO list {$$ = newflow('W', $2, $4, NULL);}
	| 	exp
		;

list: /*nothing */ {$$ = NULL;}
	|	stmt ';' list { 
    if ($3 == NULL)
	$$ = $1;
    else
	$$ = newast('L', $1, $3);
}
;                           

exp: 		exp CMP exp { $$ = newcmp($2, $1, $3);}
	| 	exp '+' exp { $$ = newast('+', $1, $3);} 
	| 	exp '-' exp { $$ = newast('-', $1, $3);} 
	| 	exp '*' exp { $$ = newast('*', $1, $3);} 
	| 	exp '/' exp { $$ = newast('/', $1, $3);} 
	| '|' 	exp {$$ = newast('|', $2, NULL);}
	| '(' 	exp ')' {$$ = $2;}
	| '-' 	exp %prec UMINUS {$$ = newast('M', $2, NULL);}
	| 	NUMBER {$$ = newnum($1);}
	| 	NAME {$$=newref($1);}
	| 	NAME '=' exp {$$ = newasgn($1, $3);}
	| 	FUNC '(' explist ')' { $$ = newfunc($1, $3);}
	| 	NAME '(' explist ')' { $$ = newcall($1, $3);}
		;

explist: 	exp
	| 	exp ',' explist {$$ = newast('L', $1, $3);}
		;

symlist: 	NAME { $$ = newsymlist($1, NULL);}
	| 	NAME ',' symlist {$$=newsymlist($1, $3);}
		;

calclist:  /* 空 */
	| 	calclist LET NAME '(' symlist ')' '=' list EOL 	{
                    doref($3, $5, $8);
                    printf("Defined %s\n> ", $3->name); 
		}
	| 	calclist stmt EOL {
                    printf("=%4.4g\n> ", eval($2));
                    treefree($2);
		}
	| 	calclist error EOL 	{
                    yyerrok;
                    printf("> ");
                }
		;


%%

/*
%type <a> exp factor term
%%
calclist:
	|	calclist exp EOL {
    printf("= %4.4g\n", eval($2));
    treefree($2);
    printf("> ");
 }
	|	calclist EOL {
    printf("> "); } 
		;

exp:		factor
	|	exp '+' factor {
    $$ = newast('+', $1, $3);
 }
	|	exp '-' factor {
    $$ = newast('-', $1, $3);
 }

factor:		term
	|	factor '*' term { $$ = newast('*', $1, $3); }
	|	factor '/' term { $$ = newast('/', $1, $3); }
	;

term:		 NUMBER {$$ = newnum($1); }
	|	'|' term {$$ = newast('|', $2, NULL); }
	|	'(' exp ')' {$$ = $2;}
	|	'-'term { $$ = newast('M', $2, NULL); }
	;

%%

*/
