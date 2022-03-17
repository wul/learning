extern int yylineno;
extern int yylex();
extern void yyerror(const char *s, ...);

struct symbol
{
    char           *name;
    double          value;
    struct ast     *func;
    struct symlist *syms;
};

struct symlist
{
    struct symbol  *sym;
    struct symlist *next;
};


/* built-in functions */
enum bifs
{
    B_sqrt = 1,
    B_exp,
    B_log,
    B_print
};
struct ast
{
    int nodetype;
    struct ast *l;
    struct ast *r;
};

/* nodetype
 * + - * / |
 * 0-7 比较操作符，位编码:  04 等于, 02小于， 01大于  1-6?
 * M 单目符号
 * L 表达式或者语句列表
 * I IF语句
 * W WHILE 语句
 * N 符号引用
 * = 赋值
 * S 符号列表
 * F 内置函数调用
 * C 用户函数调用
 */

/* 内置函数 */
struct fncall
{
    int         nodetype;
    struct ast *l;
    enum bifs   functype;
};

/* 用户自定义函数 */
struct ufncall
{
    int nodetype;
    struct ast *l;
    struct symbol *s;
};

/* 类型 I或者 W */
struct flow
{
    int nodetype;
    struct ast *cond;
    struct ast *tl;
    struct ast *el;
};

/* 类型K */
struct numval
{
    int nodetype;
    double number;
};

/* 类型 N */
struct symref
{
    int nodetype;
    struct symbol *s;
};

/* 类型 赋值 = */
struct symasgn
{
    int nodetype;
    struct symbol *s;
    struct ast *v;
};

#define NHASH 9997
struct symbol symtab[NHASH];
struct symbol *lookup(char *);

struct symlist *newsymlist(struct symbol *sym, struct symlist *next);
void freesymlist(struct symlist *sl);

struct ast *newast(int nodetype, struct ast *l, struct ast *r);
struct ast *newcmp(int cmptype, struct ast *l, struct ast *r);
struct ast *newfunc(int functype, struct ast *l);
struct ast *newcall(struct symbol *s, struct ast *l);
struct ast *newref(struct symbol *s);
struct ast *newasgn(struct symbol *s, struct ast *v);
struct ast *newflow(int nodetype, struct ast *cond, struct ast *tl, struct ast *tr);
struct ast *newnum(double d);
void doref(struct symbol *name, struct symlist *syms, struct ast *func);

double eval(struct ast *a);
void treefree(struct ast *a);
