# Observability Tools



### mount

```bash
$ mount
/dev/nvme0n1p1 on / type ext4 (rw,relatime,discard)
devtmpfs on /dev type devtmpfs (rw,relatime,size=986036k,nr_inodes=246509,mode=755)
sysfs on /sys type sysfs (rw,nosuid,nodev,noexec,relatime)
proc on /proc type proc (rw,nosuid,nodev,noexec,relatime)
securityfs on /sys/kernel/security type securityfs (rw,nosuid,nodev,noexec,relatime)
tmpfs on /dev/shm type tmpfs (rw,nosuid,nodev)
[...]
```

The first line shows that an ext4 file system stored on `/dev/nvme0n1p1` is mounted on /, with the mount flags rw, relatime, and discard. relatime is a performance improving option that reduces inode access time updates, and the subsequent disk I/O cost, by only updating the access time when the modify or change times are also being updated, or if the last update was more than a day ago.

---

### free

```bash
$ free -m
               total        used        free      shared  buff/cache   available
Mem:            7284        2127        2795         781        2360        4074
Swap:          15941          32       15909

$ free -mw
               total        used        free      shared     buffers       cache   available
Mem:            7284        2133        2787         784         132        2230        4067
Swap:          15941          32       15909
```

The wide output shows a buffers column for the buffer cache size, and a cached column for the
page cache size. The default output combines these as buff/cache.



