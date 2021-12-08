#include <stdio.h>
int array[] = {1,2,3,4,5,6,7};

#define TOTAL_ELEMENTS  (sizeof(array)/sizeof(array[0]))
void main() {
  int d = -1, x=-2;
  unsigned v = 7;

  /*
  if (d <= (TOTAL_ELEMENTS -2)) {
    x = array[ d+1 ];
  }
  */

  //这里必须转换成相同类型比较才安全，并且signed（负的）转换成unsigned 是不安全的
  if (d <= (int)v) {
    x = 100;
  }  

  printf("total size is %d and x is %d, array[0] is %d\n", TOTAL_ELEMENTS, x, array[d+1]);
}
