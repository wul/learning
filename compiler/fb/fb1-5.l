%{
#include "fb1-5.tab.h"
/*
enum yytokentype {
    NUMBER=258,
    ADD = 259,
    SUB = 260,
    MUL = 261,
    DIV = 262,
    EOL = 263,
};
*/
int yylval;

%}

%%
"+"    {return ADD; }
"-"    {return SUB; } 
"*"    {return MUL; }
"/"    {return DIV; }
[0-9]+ {yylval = atoi(yytext); return NUMBER; }
\n     {return EOL; }
[ \t]  {/* skip white spaces */}
.      {printf("Mystery character %s\n", yytext);}
%%

