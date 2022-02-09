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

  strcpy(ifr.ifr_name, argv[1]);
  ioctl(s, SIOCGIFINDEX, &ifr);
  addr.can_family = AF_CAN;
  addr.can_ifindex = ifr.ifr_ifindex;
  struct can_filter rfilter[1];
  addr.can_ifindex = ifr.ifr_ifindex;
  bind(s, (struct sockaddr*)&addr, sizeof(addr));


//rfilter[0].can_id = 0x123;
  rfilter[0].can_mask = CAN_EFF_MASK;
  setsockopt(s, SOL_CAN_RAW, CAN_RAW_FILTER, &rfilter, sizeof(rfilter));
  int v = 1;
  int r =setsockopt(s, SOL_CAN_RAW, CAN_RAW_FD_FRAMES, &v, sizeof(v));
  printf("setsockopt CAN_RAW_FD_FRAMES to 1 returned %d, errno=%d\n", r, errno);
  printf("sizeof(canfd_frame)=%d; sizeof(can_frame)=%d\n", sizeof(struct canfd_frame), sizeof(struct can_frame));
  while (1) {
    
    struct canfd_frame frame;
    int nbytes = read(s, &frame, sizeof(frame));
    canid_t canid;

    if (nbytes > 0) {
      
      int eff = (CAN_EFF_FLAG & frame.can_id) !=0 ? 1:0;
      if (eff) {
	canid = frame.can_id & CAN_EFF_MASK;
      }
      
      int canfd = 0;
      if (nbytes == CANFD_MTU) {
	canfd = 1;
      }

      printf("Received %2d bytes: CANFD=%d can_id=0x%X id=0x%X DLC=%0.2d eff=%d flags=%d \ndata[0]=0x", nbytes, canfd, frame.can_id, canid, frame.len, frame.flags, eff);
	
      for (int i = 0; i < frame.len; i++ )
	{
	  printf("%02X", frame.data[i]);
	}
      printf("\n");
      fflush(stdout);
    }
  }
  close(s);
  return 0;
}
    

