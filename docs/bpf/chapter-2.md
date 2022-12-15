# Chapter 2

#### technology Background

------

1. **BPF** was originally developed for the **BSD** operation system.

2. **BPF** works in an interesting way: A filter expression is defined by the end user using an instruction set for a BPF virtual machine (sometimes called the BPF byte-code) and then passed to the kernel for execution by a interpreter.

3. (pros) this method allows filtering to occur in the kernel level without costly copies of each packet going to the user-level processes.(improving performance)

4. (pros) it also provide safety, as filters from user space can be verified as being safe before execution.

5. Extended BPF(**eBPF**) added more registers, switched from 32-bit to 64-bit words, created flexible BPF "map" storage, and allowed calls to some restricted kernel functions. it was also designed to be be JITed with a one-to-one mapping instructions and registers, allowing prior native instruction optimization techniques to be reused for BPF. the BPF verifier was also updated to handle these extensions and reject any unsafe code.

6. **BPF** programs can execute custom latency calculations and statistical  summaries.

7. what makes **BPF** different is that it is also efficient and production environments withouts needing to add any new kernel components!.

   ```bash
   bitehist #shows the size of disk I/O as a histogram
   ```

8. **BPF** can be programmed via one of the many front end available. The main ones for tracing are,from lowest-to-highest-level language.

   - LLVM IR
   - BCC
   - bpftrace

9. bpftool(8) was added in linux 4.15 for viewing and manipulation BPF objects,including programs and maps.

10. the `bpftool perf` subcommand shows BPF programs attached via `perf_event_open()`,witch is the norm for BCC and bpftrace programs on linux 4.17 and later.

    ```bash
    sudo apt-get install binutils-dev
    sudo apt-get install libreadline-dev
    cd <linux-source-directory>/tools/bpf/
    make
    ```

11. 