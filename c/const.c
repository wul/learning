#include <stdio.h>
/* const后面如果是指针，修饰的是指针指向的内容不可修改，不是指针本身 */ 
void change(const char *p) {
  
  p = "aaaa";

  
}
int main() {

    //a 变量不可改变
  const int a = 0;
  const char *p = "abcde";
  //p[0] =  "d";
  p = "def";
  printf("%s\n", p);


  change(p);

  /* a cannot change */
  //a = 1;
  return 0;
}
