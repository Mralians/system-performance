# Loading the Program into the Kernel

1. The following is an example of using `pbftool` to load a program into the kernel.
   ```bash
   bpftool prog load hello.bpf.o /sys/fs/bpf/hello
   ```

   This load the eBPF program from our compiled object file and "pins" it to the location `/sys/fs/bpf/hello`.

   ```bash
   $ ls /sys/fs/bpf
   hello
   ```

