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

