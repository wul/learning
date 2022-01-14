extern int yylineno;

struct ast {
  int nodetype;
  struct ast *l;
  struct ast *r;
};


struct numval {
  int nodetype;
  double number;
};

struct ast *newast(int nodetype, struct ast *l, struct ast* r);
struct ast *newnum(double d);

double eval(struct ast *a);
void treefree(struct ast* a);


  
