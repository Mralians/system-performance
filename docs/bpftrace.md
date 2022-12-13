## CPUs

- Trace new processes with arguments:

```bash
bpftrace -e 'tracepoint:syscalls:sys_enter_execve { join(args->argv); }'
```

- Count syscalls by process:

```bash
bpftrace -e 'tracepoint:raw_syscalls:sys_enter { @[pid, comm] = count(); }'
```

- Count syscalls by syscall probe name:

```bash
bpftrace -e 'tracepoint:syscalls:sys_enter_* { @[probe] = count(); }'
```

- Sample running process names at 99 Hertz:

```bash
bpftrace -e 'profile:hz:99 { @[comm] = count(); }'
```

- Sample user and kernel stacks at 49 Hertz, system wide, with the process name:

```bash
bpftrace -e 'profile:hz:49 { @[kstack, ustack, comm] = count(); }'
```

- Sample user-level stacks at 49 Hertz, for PID 189:

```bash
bpftrace -e 'profile:hz:49 /pid == 189/ { @[ustack] = count(); }'
```

- Sample user-level stacks 5 frames deep at 49 Hertz, for PID 189:

```bash
bpftrace -e 'profile:hz:49 /pid == 189/ { @[ustack(5)] = count(); }'804
```

- Sample user-level stacks at 49 Hertz, for processes named “mysqld”:

```bash
bpftrace -e 'profile:hz:49 /comm == "mysqld"/ { @[ustack] = count(); }'
```

- Count kernel CPU scheduler tracepoints:

```bash
bpftrace -e 'tracepont:sched:* { @[probe] = count(); }'
```

- Count off-CPU kernel stacks for context switch events:

```bash
bpftrace -e 'tracepont:sched:sched_switch { @[kstack] = count(); }'
```

- Count kernel function calls beginning with “vfs_”:

```bash
bpftrace -e 'kprobe:vfs_* { @[func] = count(); }'
```

- Trace new threads via pthread_create():

```bash
bpftrace -e 'u:/lib/x86_64-linux-gnu/libpthread-2.27.so:pthread_create {printf("%s by %s (%d)\n", probe, comm, pid); }'
```



## Memory

- Sum libc malloc() request bytes by user stack and process (high overhead):

```bash
bpftrace -e 'u:/lib/x86_64-linux-gnu/libc.so.6:malloc {@[ustack, comm] = sum(arg0); }'
```

- Sum libc malloc() request bytes by user stack for PID 181 (high overhead):

```bash
bpftrace -e 'u:/lib/x86_64-linux-gnu/libc.so.6:malloc /pid == 181/ {@[ustack] = sum(arg0); }'
```

- Show libc malloc() request bytes by user stack for PID 181 as a power-of-2 histogram (high overhead):

```bash
bpftrace -e 'u:/lib/x86_64-linux-gnu/libc.so.6:malloc /pid == 181/ {@[ustack] = hist(arg0); }'
```

- Sum kernel kmem cache allocation bytes by kernel stack trace:

```bash
bpftrace -e 't:kmem:kmem_cache_alloc { @bytes[kstack] = sum(args->bytes_alloc); }'
```

- Count process heap expansion (brk(2)) by code path:

```bash
bpftrace -e 'tracepoint:syscalls:sys_enter_brk { @[ustack, comm] = count(); }'
```

- Count page faults by process:

```bash
bpftrace -e 'software:page-fault:1 { @[comm, pid] = count(); }'bpftrace One-Liners
```

- Count user page faults by user-level stack trace:

```bash
bpftrace -e 't:exceptions:page_fault_user { @[ustack, comm] = count(); }'
```

- Count vmscan operations by tracepoint:

```bash
bpftrace -e 'tracepoint:vmscan:* { @[probe]++; }'
```

- Count swapins by process:

```bash
bpftrace -e 'kprobe:swap_readpage { @[comm, pid] = count(); }'
```

- Count page migrations:

```bash
bpftrace -e 'tracepoint:migrate:mm_migrate_pages { @ = count(); }'
```

- Trace compaction events:

```bash
bpftrace -e 't:compaction:mm_compaction_begin { time(); }'
```

- List USDT probes in libc:

```bash
bpftrace -l 'usdt:/lib/x86_64-linux-gnu/libc.so.6:*'
```

- List kernel kmem tracepoints:

```bash
bpftrace -l 't:kmem:*'
```

- List all memory subsystem (mm) tracepoints:

```bash
bpftrace -l 't:*:mm_*'
```



## File Systems

- Trace files opened via openat(2) with process name:

```bash
bpftrace -e 't:syscalls:sys_enter_openat { printf("%s %s\n", comm,str(args->filename)); }'
```

- Count read syscalls by syscall type:

```bash
bpftrace -e 'tracepoint:syscalls:sys_enter_*read* { @[probe] = count(); }'
```

- Count write syscalls by syscall type:

```bash
bpftrace -e 'tracepoint:syscalls:sys_enter_*write* { @[probe] = count(); }'
```

- Show the distribution of read() syscall request sizes:

```bash
bpftrace -e 'tracepoint:syscalls:sys_enter_read { @ = hist(args->count); }'
```

- 
  Show the distribution of read() syscall read bytes (and errors):

```bash
bpftrace -e 'tracepoint:syscalls:sys_exit_read { @ = hist(args->ret); }'
```

- Count read() syscall errors by error code:

```bash
bpftrace -e 't:syscalls:sys_exit_read /args->ret < 0/ { @[- args->ret] = count(); }'
```

- Count VFS calls:

```bash
bpftrace -e 'kprobe:vfs_* { @[probe] = count(); }'
```

- Count VFS calls for PID 181:

```bash
bpftrace -e 'kprobe:vfs_* /pid == 181/ { @[probe] = count(); }'
```

- Count ext4 tracepoints:

```bash
bpftrace -e 'tracepoint:ext4:* { @[probe] = count(); }'
```

- Count xfs tracepoints:

```bash
bpftrace -e 'tracepoint:xfs:* { @[probe] = count(); }'
```

- Count ext4 file reads by process name and user-level stack:

```bash
bpftrace -e 'kprobe:ext4_file_read_iter { @[ustack, comm] = count(); }'
```

- Trace ZFS spa_sync() times:

```bash
bpftrace -e 'kprobe:spa_sync { time("%H:%M:%S ZFS spa_sync()\n"); }'
```

- Count dcache references by process name and PID:

```bash
bpftrace -e 'kprobe:lookup_fast { @[comm, pid] = count(); }'
```



## Disks

- Count block I/O tracepoints events:

```bash
bpftrace -e 'tracepoint:block:* { @[probe] = count(); }'
```

- Summarize block I/O size as a histogram:

```bash
bpftrace -e 't:block:block_rq_issue { @bytes = hist(args->bytes); }'
```

- Count block I/O request user stack traces:

```bash
bpftrace -e 't:block:block_rq_issue { @[ustack] = count(); }'
```

- Count block I/O type flags:

```bash
bpftrace -e 't:block:block_rq_issue { @[args->rwbs] = count(); }'bpftrace One-Liners
```

- Trace block I/O errors with device and I/O type:

```bash
bpftrace -e 't:block:block_rq_complete /args->error/ {printf("dev %d type %s error %d\n", args->dev, args->rwbs, args->error); }'
```

- Count SCSI opcodes:

```bash
bpftrace -e 't:scsi:scsi_dispatch_cmd_start { @opcode[args->opcode] = count(); }'
```

- Count SCSI result codes:

```bash
bpftrace -e 't:scsi:scsi_dispatch_cmd_done { @result[args->result] = count(); }'
```

- Count SCSI driver function calls:

```bash
bpftrace -e 'kprobe:scsi* { @[func] = count(); }'
```



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

