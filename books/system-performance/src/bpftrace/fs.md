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

