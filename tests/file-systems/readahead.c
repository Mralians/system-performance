#define _GNU_SOURCE

#include <err.h>
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <unistd.h>

#define BUFF_SIZE 125
int main(int argc, char *argv[]) {
  char *filename = "/etc/passwd";
  char buf[BUFF_SIZE];
  ssize_t numRead, numWriten;

  int fd = open(filename, O_RDONLY);
  if (fd == -1)
    errx(EXIT_FAILURE, "open()");

  int ret = readahead(fd, 0, BUFF_SIZE);
  if (ret == -1)
    errx(EXIT_FAILURE, "readahead()");

  while ((numRead = read(fd, buf, sizeof(buf))) > 0) {
    numWriten = write(STDOUT_FILENO, buf, numRead);
    if (numWriten != numRead)
      errx(EXIT_FAILURE, "Unable to write the entire buffer");
    if (numWriten == -1)
      errx(EXIT_FAILURE, "write()");
  }
  exit(EXIT_SUCCESS);
}
