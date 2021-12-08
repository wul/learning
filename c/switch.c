#include <stdio.h>
void main() {

  const int two = 2;

  int i = 2;
  switch (i) {
  case 1: {
    printf("case 1\n");
    break;
  }
  case two: {
    printf("case two\n");
    // 漏掉break，会fall through
    //    break;
  }

  default: {
    printf("fall through if case two not break\n");
  }
  }
}
   
