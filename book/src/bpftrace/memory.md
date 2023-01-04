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

