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

