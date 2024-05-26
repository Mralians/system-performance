#include <errno.h>
#include <linux/netlink.h>
#include <linux/rtnetlink.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

#define UEVENT_BUFFER_SIZE 2048

int main(int argc, char *argv[]) {
  int sockfd;
  struct sockaddr_nl sa;

  memset(&sa, 0, sizeof(sa));
  sa.nl_family = AF_NETLINK;
  sa.nl_groups = RTMGRP_LINK | RTMGRP_IPV4_IFADDR;

  sockfd = socket(AF_NETLINK, SOCK_RAW, NETLINK_KOBJECT_UEVENT);
  if (sockfd < 0) {
    perror("socket");
    return -1;
  }

  if (bind(sockfd, (struct sockaddr *)&sa, sizeof(sa)) < 0) {
    perror("bind");
    close(sockfd);
    return -1;
  }

  while (1) {
    char buf[UEVENT_BUFFER_SIZE];
    int len = recv(sockfd, buf, sizeof(buf), 0);
    if (len < 0) {
      perror("recv");
      close(sockfd);
      return -1;
    }
    buf[len] = '\0';

    printf("Received uevent:\n%s\n", buf);
  }

  close(sockfd);
  return 0;
}
