%option noyywrap nodefault yylineno

%x COMMENT
%x IFILE

UCN  (\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8})
EXP  ([Ee][-+]?\d+)
ILEN ([Uu](L|l|LL|ll)?|(L|l|LL|ll)[Uu]?)

%{
  struct symbol {
    struct ref *reflist;
    char       *name;
  };
  
  struct ref {
    struct ref *next;
    char       *filename;
    int         flags;      /*指示这是一个定义还是引用 */
    int         lineno;
  };
  
  #define MHASH 9997
  struct symbol symtab[MHASH];
  struct symbol *lookup(char*);

  int defining;  
  void addref(int, char*, char*, int);
  
  char *curfilename;
  struct bufstack {
    struct bufstack *prev;
    YY_BUFFER_STATE bs;
    int             lineno;
    char           *filename;
    FILE           *f;
  } *curbs;
  
  int newfile(char *fn);
  int popfile(void);
  

%}


%%
"/*"          { BEGIN COMMENT; } 
<COMMENT>"*/" { BEGIN INITIAL; }
<COMMENT>([^*]|\n)+|. 
<COMMENT><<EOF>>   { printf("%s:%d: Unterminated comment\n", curfilename, yylineno); return 0; }

"//".*\n


_Bool |
_Complex |
_Imaginary |
auto |
char |
const |
double |
enum |
extern |
float |
inline |
int |
long |
register |
restrict |
short |
signed |
static |
struct |
typedef |
union |
unsigned |
void |
volatile {defining = 1; }

break
case
continue
default
do
else
for
goto
if
return
sizeof
switch
while


0[0-7]*{ILEN}?
[1-9][0-9]*{ILEN}?

(\d*\.\d+|\d+.){EXP}?[flFL]?
[0-9]+{EXP}[flFL]?

0[Xx]([0-9a-fA-F]*\.[0-9a-fA-F]+|[0-9a-fA-F]+\.?)[Pp][-+]?[0-9]+[flFL]?



\'([^'\\]|\\['"?\\abfnrtv]|\\[0-7]{1,3}|\\[Xx][0-9a-fA-F]+|{UCN})+\'

L?\"([^"\\]|\\['"?\\abfnrtv]|\\[0-7]{1,3}|\\[Xx][0-9a-fA-F]+|{UCN})*\"


"{"|"<%"|";"                                    { defining = 0; }
"["|"]"|"("|")"|"{"|"}"|"."|"->"
"++"|"--"|"&"|"*"|"+"|"-"|"~"|"!"
"/"|"%"|"<<"|">>"|"<"|">"|"<="|">="|"=="|"!="|"^"|"|"|"&&"|"||"
"?"|":"|";"|"..."
"="|"*="|"/="|"%="|"+="|"-="|"<<="|">>="|"&="|"^=""|="
","|"#"|"##"
"<:"|":>"|"%>"|"%:"|"%:%:"


([_a-zA-Z]|{UCN})([_a-zA-Z0-9]|{UCN})* {addref(yylineno, curfilename, yytext, defining);}

[ \t\n]+
\\$

"#"" "*if.*\n
"#"" "*else.*\n
"#"" "*endif.*\n
"#"" "*define.*\n 
"#"" "*line.*\n


^"#"[ \t]*include[ \t]*[\"<] { BEGIN IFILE; }
<IFILE>[^>\"]+               {
                		{ int c;
				  while((c = input()) && c != '\n') ;
				}
    				 newfile(strdup(yytext)); BEGIN INITIAL;
				 BEGIN INITIAL;
			     }

<IFILE>.|\n		{fprintf(stderr, "%s:%d bad include line\n", curfilename, yylineno);
			BEGIN INITIAL;
			}

<<EOF>>			{if(!popfile()) yyterminate();}


. 			{ printf("%s:%d: Mystery character '%s'\n", curfilename, yylineno, yytext);}

%%

static int symcompare(const void *xa, const void *xb)
{
  const struct symbol *a = xa;
  const struct symbol *b = xb;
  if(!a->name) {
    if(!b->name) return 0;
    return 1;
  }
  if(!b->name) return -1;
  return strcmp(a->name, b->name);
}

void printrefs()
{
  struct symbol *sp;
  qsort(symtab, MHASH, sizeof(struct symbol), symcompare); /* sort the symbol table */
  for(sp = symtab; sp->name && sp < symtab+MHASH; sp++) {
    char *prevfn = NULL; /* last printed filename, to skip dups */
    /* reverse the list of references */ struct ref *rp = sp->reflist;
    struct ref *rpp = 0; /* previous ref */
    struct ref *rpn;
    do {
      rpn = rp->next; rp->next = rpp; rpp = rp;
      rp = rpn;
    } while(rp);
    /* next ref */
    /* now print the word and its references */
    printf("%10s", sp->name);
    for(rp = rpp; rp; rp = rp->next) {
      if(rp->filename == prevfn) {
	printf(" %d", rp->lineno);
      } else {
	printf(" %s:%d", rp->filename, rp->lineno);
	prevfn = rp->filename;
      }
      if(rp->flags & 01) printf("*");
    }
    printf("\n");
  }
}

int main(int argc, char **argv)
{
  int i;
  if (argc == 1) {
    fprintf(stderr, "need filename\n");
    return 1;
  }

  for (i = 1; i < argc; i++) {
    if(newfile(argv[i]))
      yylex();
  }

  printrefs();
  return 0;
}

void addref(int lineno, char* filename, char *word, int flags)
{
  struct ref *r;
  struct symbol *sp = lookup(word);

  if(sp->reflist &&
     sp->reflist->lineno == lineno &&
     sp->reflist->filename == filename) return;

  r = malloc(sizeof(struct ref));
  if(!r) {fputs("out of space\n", stderr); abort();}
  r->next = sp->reflist;
  r->filename = filename;
  r->lineno = lineno;
  r->flags = flags;
  sp->reflist = r;
}

int newfile(char *fn)
{
  FILE *f = fopen(fn, "r");
  struct bufstack *bs = malloc(sizeof(struct bufstack));

  if(!f) {perror(fn); return 0;}
  if(!bs) {perror("malloc"); exit(1);}

  if (curbs)
    curbs->lineno = yylineno;


  /* yy_create_buffer  &&
   * yy_switch_to_buffer
   */
  
  bs->prev     = curbs;
  bs->bs       = yy_create_buffer(f, YY_BUF_SIZE); /*CREATED YY_BUFFER_STATE and initialized with input file*/
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
static unsigned symhash(char *sym)
{
	unsigned int hash = 0;
	unsigned c;
	while (c = *sym++) hash = hash * 9 ^ c;
	return hash;
}

struct symbol* lookup(char* sym)
{
	struct symbol *sp = &symtab[symhash(sym) % MHASH];
	int scount = MHASH;
	while(--scount >= 0) {
	  if(sp->name && !strcmp(sp->name, sym)) return sp;

	  if(!sp->name) {
	    sp->name = strdup(sym);
	    sp->reflist = 0;
	    return sp;
	  }

	  if(++sp >= symtab+MHASH) sp = symtab;
	}
	fputs("symbol table overflow\n", stderr);
	abort();
}

