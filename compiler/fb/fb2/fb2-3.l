%option noyywrap
%x IFILE



%{
  struct bufstack {
    struct bufstack *prev;
    YY_BUFFER_STATE  bs;
    int              lineno;
    char            *filename;
    FILE            *f;
  } *curbs = 0;

  char *curfilename;
  int newfile(char *fn);
  int popfile(void);

%}

%%
^"#"[\t ]*include[ \t]+\<  { BEGIN IFILE;}
<IFILE>[^\>]+      {
                           { int c;
			     while((c = input()) && c !='\n');
			   }
			   yylineno++;
			   fprintf(yyout, "include file: %s\n", yytext);
			   if (!newfile(yytext)) 
			     yyterminate();
			   BEGIN INITIAL;
			 }
			 
<IFILE>.|\n              {
                           fprintf(stderr, "%4d bad include line\n", yylineno);
			   fprintf(stderr, "%s", yytext);
			   yyterminate();
			 }

<<EOF>>                  {
                           if(!popfile()) 
			     yyterminate();
			 }

^.                       { fprintf(yyout, "%4d %s", yylineno, yytext); }
^\n                      { fprintf(yyout, "%4d %s", yylineno++, yytext); }
\n                       { ECHO; yylineno++;}
.                        { ECHO; }


			   
%%

int main(int argc, char **argv)
{
  if (argc < 2) {
    fprintf(stderr, "need filename\n");
    return 1;
  }
 

  if (newfile(argv[1]))
    yylex();
}

int newfile(char *fn)
{
  FILE *f = fopen(fn, "r");
  struct bufstack *bs = malloc(sizeof(struct bufstack));

  if(!f) {perror(fn); return 0;}
  if(!bs) {perror("malloc"); exit(1);}

  if (curbs)
    curbs->lineno = yylineno;

  bs->prev     = curbs;
  bs->bs       = yy_create_buffer(f, YY_BUF_SIZE);
  bs->f        = f;
  bs->filename = fn;

  yy_switch_to_buffer(bs->bs);
  curbs = bs;
  yylineno = 1;
  curfilename = fn;
  return 1;
}

int popfile(void)
{
  struct bufstack *bs = curbs;
  struct bufstack *prev = curbs->prev;

  if (NULL == bs) {
    return 0;
  }

  fclose(bs->f);
  yy_delete_buffer(bs->bs);
  free(bs);
  
  if (NULL == prev)
    return 0;

  yy_switch_to_buffer(prev->bs);
  curbs = prev;
  yylineno = curbs->lineno;
  curfilename = curbs->filename;
  return 1;
}
