#include <stdio.h>
#include <unistd.h>
#include <string.h>
#include <sys/socket.h>
#include <linux/can.h>
#include <linux/can/raw.h>
#include <sys/ioctl.h>
#include <net/if.h>
#include <errno.h>

/* canfd support
 * sudo ip link add vcan0 type vcan
 * sudo ip link set vcan0 mtu 72 
 * sudo ip link set vcan0 up
 */
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
  //setsockopt(s, SOL_CAN_RAW, CAN_RAW_FILTER, &rfilter, sizeof(rfilter));
  int v = 1;
  int r =setsockopt(s, SOL_CAN_RAW, CAN_RAW_FD_FRAMES, &v, sizeof(v));
  printf("setsockopt CAN_RAW_FD_FRAMES to 1 returned %d, errno=%d\n", r, errno);
  printf("sizeof(canfd_frame)=%d; sizeof(can_frame)=%d", sizeof(struct canfd_frame), sizeof(struct can_frame));
  while (1) {
    
    struct canfd_frame frame;
    int nbytes = read(s, &frame, sizeof(frame));


    if (nbytes > 0) {
      
      printf("Received %2d bytes: ID=0x%X DLC=%0.2d data[0]=0x", nbytes, frame.can_id, frame.len);
      for (int i = 0; i < frame.len; i++ )
	{
	  printf("%02X", frame.data[i]);
	}
      printf("\n");
    }
  }
  close(s);
  return 0;
}
    

