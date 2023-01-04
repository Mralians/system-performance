## Networking

- Count socket accept(2)s by PID and process name:

```bash
bpftrace -e 't:syscalls:sys_enter_accept* { @[pid, comm] = count(); }'
```

- Count socket connect(2)s by PID and process name:

```bash
bpftrace -e 't:syscalls:sys_enter_connect { @[pid, comm] = count(); }'
```

- Count socket connect(2)s by user stack trace:

```bash
bpftrace -e 't:syscalls:sys_enter_connect { @[ustack, comm] = count(); }'
```

- Count socket send/receives by direction, on-CPU PID, and process name:

```bash
bpftrace -e 'k:sock_sendmsg,k:sock_recvmsg { @[func, pid, comm] = count(); }'
```

- Count socket send/receive bytes by on-CPU PID and process name:

```bash
bpftrace -e 'kr:sock_sendmsg,kr:sock_recvmsg /(int32)retval > 0/ { @[pid, comm] = sum((int32)retval); }'
```

- Count TCP connects by on-CPU PID and process name:

```bash
bpftrace -e 'k:tcp_v*_connect { @[pid, comm] = count(); }'
```

- Count TCP accepts by on-CPU PID and process name:

```bash
bpftrace -e 'k:inet_csk_accept { @[pid, comm] = count(); }'
```

- Count TCP send/receives by on-CPU PID and process name:

```bash
bpftrace -e 'k:tcp_sendmsg,k:tcp_recvmsg { @[func, pid, comm] = count(); }'
```

- TCP send bytes as a histogram:

```bash
bpftrace -e 'k:tcp_sendmsg { @send_bytes = hist(arg2); }'
```

- TCP receive bytes as a histogram:

```bash
bpftrace -e 'kr:tcp_recvmsg /retval >= 0/ { @recv_bytes = hist(retval); }'
```

- Count TCP retransmits by type and remote host (assumes IPv4):

```bash
bpftrace -e 't:tcp:tcp_retransmit_* { @[probe, ntop(2, args->saddr)] = count(); }'
```

- Count all TCP functions (adds high overhead to TCP):

```bash
bpftrace -e 'k:tcp_* { @[func] = count(); }'
```

- Count UDP send/receives by on-CPU PID and process name:

```bash
bpftrace -e 'k:udp*_sendmsg,k:udp*_recvmsg { @[func, pid, comm] = count(); }'
```

- UDP send bytes as a histogram:

```bash
bpftrace -e 'k:udp_sendmsg { @send_bytes = hist(arg2); }'
```

- UDP receive bytes as a histogram:

```bash
bpftrace -e 'kr:udp_recvmsg /retval >= 0/ { @recv_bytes = hist(retval); }'
```

- Count transmit kernel stack traces:

```bash
bpftrace -e 't:net:net_dev_xmit { @[kstack] = count(); }'
```

- Show receive CPU histogram for each device:

```bash
bpftrace -e 't:net:netif_receive_skb { @[str(args->name)] = lhist(cpu, 0, 128, 1); }'
```

- Count ieee80211 layer functions (adds high overhead to packets):

```bash
bpftrace -e 'k:ieee80211_* { @[func] = count()'
```

- Count all ixgbevf device driver functions (adds high overhead to ixgbevf):

```bash
bpftrace -e 'k:ixgbevf_* { @[func] = count(); }'
```

- Count all iwl device driver tracepoints (adds high overhead to iwl):

```bash
bpftrace -e 't:iwlwifi:*,t:iwlwifi_io:* { @[probe] = count(); }'
```

