#include <stdio.h>
#include <stdlib.h>
typedef unsigned char uint8_t;


static const uint8_t dlc2len[] = {
    0, 1, 2, 3, 4, 5, 6, 7,
    8, 12, 16, 20, 24, 32, 48, 64
};

/* get data length from can_dlc with sanitized can_dlc */
uint8_t can_dlc2len(uint8_t can_dlc)
{
    return dlc2len[can_dlc & 0x0F];
}

static const uint8_t len2dlc[] = {
    0, 1, 2, 3, 4, 5, 6, 7, 8,                              /* 0 - 8 */
    9, 9, 9, 9,                                             /* 9 - 12 */
    10, 10, 10, 10,                                         /* 13 - 16 */
    11, 11, 11, 11,                                         /* 17 - 20 */
    12, 12, 12, 12,                                         /* 21 - 24 */
    13, 13, 13, 13, 13, 13, 13, 13,                         /* 25 - 32 */
    14, 14, 14, 14, 14, 14, 14, 14,                         /* 33 - 40 */
    14, 14, 14, 14, 14, 14, 14, 14,                         /* 41 - 48 */
    15, 15, 15, 15, 15, 15, 15, 15,                         /* 49 - 56 */
    15, 15, 15, 15, 15, 15, 15, 15                          /* 57 - 64 */
};

/* map the sanitized data length to an appropriate data length code */
uint8_t can_len2dlc(uint8_t len)
{
    if (len > 64) {
        return 0xF;
    }

    return len2dlc[len];
}



int main(int argc, char **argv) {
  printf("%-8s %-8s %-8s\n", "Length", "DLC", "Length2");

  uint8_t saved = 0;
  for(int len=0; len < 65; len++) {
    uint8_t dlc = can_len2dlc(len);
    if (dlc != saved) {
      printf("-------------------------------\n");
      saved = dlc;
    }
    printf("%-8d %-8d %-8d\n", len, can_len2dlc(len), can_dlc2len(can_len2dlc(len)));
  }
  
  return 0;
}
    
