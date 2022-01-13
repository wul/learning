#include <stdio.h>
#include <unistd.h>
#include <string.h>
#include <sys/socket.h>
#include <linux/can.h>
#include <linux/can/raw.h>
#include <sys/ioctl.h>
#include <net/if.h>
#include <errno.h>

int main(int argc, char **argv)
{
  int s;
  struct sockaddr_can addr;
  struct ifreq        ifr;
  int nbytes;
  s = socket(PF_CAN, SOCK_RAW, CAN_RAW);

  strcpy(ifr.ifr_name, "vcan0");
  ioctl(s, SIOCGIFINDEX, &ifr);
  addr.can_family = AF_CAN;
  addr.can_ifindex = ifr.ifr_ifindex;

  bind(s, (struct sockaddr*)&addr, sizeof(addr));

  //disable filter rule, only sending packet
  setsockopt(s, SOL_CAN_RAW, CAN_RAW_FILTER, NULL, 0);
  struct can_frame frame;
  frame.can_id = 0x123;
  frame.can_dlc = 1;
  frame.data[0] = 'Y';

  nbytes = write(s, &frame, sizeof(frame));
  if (nbytes != sizeof(frame)) {
    printf("write error, write returned:%d, errno=%d\n", nbytes, errno);
  }

  return 0;
}
  
