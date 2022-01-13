#include <stdio.h>
#include <unistd.h>
#include <string.h>
#include <sys/socket.h>
#include <linux/can.h>
#include <linux/can/raw.h>
#include <sys/ioctl.h>
#include <net/if.h>
int main(int argc, char **argv)
{
  int s;
  struct sockaddr_can addr;
  struct ifreq        ifr;

  s = socket(PF_CAN, SOCK_RAW, CAN_RAW);

  strcpy(ifr.ifr_name, "vcan0");
  ioctl(s, SIOCGIFINDEX, &ifr);
  addr.can_family = AF_CAN;
  addr.can_ifindex = ifr.ifr_ifindex;
  struct can_filter rfilter[1];
  addr.can_ifindex = ifr.ifr_ifindex;
  bind(s, (struct sockaddr*)&addr, sizeof(addr));


  rfilter[0].can_id = 0x123;
  rfilter[0].can_mask = CAN_SFF_MASK;
  setsockopt(s, SOL_CAN_RAW, CAN_RAW_FILTER, &rfilter, sizeof(rfilter));
  while (1) {
    
    struct can_frame frame;
    int nbytes = read(s, &frame, sizeof(frame));


    if (nbytes > 0) {
      printf("ID=0x%X DLC=%d data[0]=0x%X\n", frame.can_id, frame.can_dlc, frame.data[0]);
    }
  }
  close(s);
  return 0;
}
    
