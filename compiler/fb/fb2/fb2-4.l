%option noyywrap nodefault yylineno case-insensitive

%{
	struct symbol {
	       char *name;
	       struct ref *reflist;
	};

	struct ref {
	       struct ref *next;
	       char       *filename;
	       int 	   flags;
	       int	   lineno;
	};
	#define MHASH 9997
	struct symbol symtab[MHASH];

	struct symbol *lookup(char*);
	void   addref(int, char*, char*, int);
	char  *curfilename;
	
%}


%%
a |
an |
and |
are |
as |
at |
be |
but |
for |
in |
is |
it |
of |
on |
or |
that |
the |
this |
to

[a-z]+(\'(s|t))?	{ addref(yylineno, curfilename, yytext, 0); }
.|\n
%%

static unsigned symhash(char *sym)
{
	unsigned int hash = 0;
	unsigned c;
	while (c = *sym++) hash = hash * 9 ^ c;
        printf("Hash value for %s is %d\n", sym, hash);
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
  qsort(symtab, MHASH, sizeof(struct symbol), symcompare);
  for(sp = symtab; sp->name && sp < symtab+MHASH; sp++) {
    char *prevfn = NULL;
    
    struct ref *rp = sp->reflist;
    struct ref *rpp = 0;
    struct ref *rpn;
    
    do {
      rpn = rp->next;
      rp->next = rpp;
      rpp = rp;
      rp  = rpn;
    } while(rp);
    
    printf("%10s", sp->name);
    for(rp = rpp; rp; rp=rp->next) {
      if(rp->filename == prevfn) {
	printf(" %d", rp->lineno);
      } else {
	printf(" %s:%d", rp->filename, rp->lineno);
	prevfn = rp->filename;
      }
    }
    printf("\n");
  }
}

int main(int argc, char **argv)
{
  int i;
  if (argc < 2) {
    curfilename = "(stdin)";
    yylineno = 1;
    yylex();
  } else {
    for (i = 1; i < argc; i++) {
      FILE *f = fopen(argv[i], "r");
      if (!f) {
	perror(argv[1]);
	return 1;
      }
      curfilename = argv[1];
      yyrestart(f);
      yylineno = 1;
      yylex();
      fclose(f);
    }
    
    printrefs();
    return 0;
  }
}
