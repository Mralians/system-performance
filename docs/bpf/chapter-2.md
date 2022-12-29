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
    bpftool prog dump xlated id 263 visual > biolatency_done.dot #graphViz
    dot -Tpng -Elen=2.5 biolatency_done.dot -o biolatency_done.png
    ```

11. The `prog dump jited` subcommand shows the machine code for the processor that is executed.

12. The `btf dump id <id-number>` shows the BTF IDs.

13. A **BPF** program cannot call arbitrary kernel functions. to accomplish certain tasks with this limitation, "helper" functions that BPF can call have been provided.

    ​																		**BPF Helper functions**

    | BPF Helper Function                                | Description                                                  |
    | :------------------------------------------------- | ------------------------------------------------------------ |
    | `bpf_map_lookup_elem(map, key)`                    | **Finds** a key in a map and returns its value(pointer).     |
    | `bpf_map_update_elem(map,key,value,flags)`         | **Update** the value of the entry selected by key.           |
    | `bpf_map_delete_elem(map, key)`                    | **Deletes** the entry selected by key from the map.          |
    | `bpf_probe_read(dst, size, src)`                   | Safely reads size bytes from address src n and stores in dst. |
    | `bpf_ktime_get_ns()`                               | Returns the time since boot,in nanoseconds.                  |
    | `bpf_trace_printk(fmt, fmt_size, ...)`             | A debugging helper that writes to TraceFs trace{_pipe}.      |
    | `bpf_get_current_pid_tgid()`                       | Returns a u64 containing the current TGID (what user space calls the PID) in the upper bits and current PID (what user space calls the kernel thread ID) in the lower bits. |
    | `bpf_perf_event_output(ctx, map, data, size)`      | Writes data to the perf_event ring buffers; this is used for per-event output. |
    | `bpf_get_stackid(ctx, map, flags)`                 | Fetches a user or kernel stack trace and returns an identifier. |
    | `bpf_get_current_task()`                           | Returns the current task struct. this contains many details about the running process and links to other structs containing system state. Note that these are all considered an unstable API. |
    | `pbf_probe_read_str(dst, size, ptr)`               | Copies a NULL terminated string from an unsafe pointer to the destination, limited by size (including the NULL byte). |
    | `bpf_perf_event_read_value(map, flags, buf, size)` | Reads a perf_event counter and stores it in the buf. This is a way to read PMCs during a BPF program. |
    | `bpf_get_current_cgroup_id()`                      | Returns the current cgroup ID.                               |
    | `bpf_spin_lock(lock)`<br />`bpf_spin_unlock(lock)` | Concurrency control for network programs.                    |
    | `bpf_current_comm(buf, buf_size)`                  | Copies the task name to the buffer.                          |

14. The term current in these descriptions refers to the currently running thread. the thread that is currently on-CPU.

15. `include/uapi/linux/bpf.h` file often provides detailed documentation for these helpers.

16. `bpf_probe_read()` is a particularly important helper. Memory access in BPF is restricted to BPF registers and the stack. Arbitrary memory(such as other kernel memory outside of BPF) must be read via `pbf_probe_read()`, witch performs safety checks and disables page faults to ensure that the reads do not cause faults from probe context (witch could cause kernel problems).

    ------

    

## Stack Trace Walking 

Stack trace are an invaluable tool for understanding the code path that led to an event, as well as profiling kernel and user code to observe where execution time is spent.

BPF provides special map types for recording stack traces and can fetch them using frame-pointer-based or ORC-base stack walks. BPF may support other stack walking techniques in the future.

1. **Frame Pointer-Based Stacks**

   The frame pointer technique follows a convention where the head of a linked list of stack frames can always be found in a register (RBP on `x86_64`) and where the return address is stored at a known offset(+8) from the stored RBP.

   The AMD64 ABI notes that the use of RBP as a frame pointer register is conventional, and can be avoided to save function prologue and epilogue instructions, and make RBP available as a general-purpose register.

   The gcc compile currently defaults to omitting the frame pointer and using RBP as a general-purpose register, which breaks frame pointer-based stack walking. This defaults can be reverted using the `-fno-omit=frame-pointer` option.

   Many microservices at Netflix are running with the frame pointer reenabled, as the performance wins found by CPU profiling outweigh the tiny loss of performance.

2. **debuginfo**

   Additional debugging information is often available for software as debuginfo packages, which contain ELF debuginfo  files in the DWARF format. these include sections that debuggers such as gdb(1) can use to walk the stack trace, even when no frame pointer registers is in use. The ELF sections are `.eh_frame` and `.debug_frame`.

   BPF does not currently support this technique of stack walking.

3. **Last Branch Record(LBR)**

   last branch record is an intel processor feature to record branches in a hardware buffer, including function call branches. this technique has no overhead and can be used to reconstruct a stack trace. However, it is limited in depth depending on the processor, and may only support recording 4 to 32 branches.

   LRB is not currently supported by BPF, but it may be in the future. A limited stack trace is better than no stack trace!

4. **ORC**

   A new debug information format that has been devised for stack traces, Oops Rewind Capability(ORC), is less processor intensive than DWARF. ORC uses `.orc_unwind` and `.orc_unwind_ip` ELF sections, and it has so far been implemented for linux kernel. On register-limited architectures, it may be desirable to compile the kernel without the frame pointer and use ORC for stack traces instead.

   ORC stack have not yet been developed for user space.

5. **Symbols**

   Stack traces are currently recorded in the kernel as an array of address that are later translated to symbols (such as function names) by a user-level program.

   ------

   

## Flame Graphs

flame graphs are visualization of stack trace.

1. **Stack trace**

   A stack trace, also called a stack back trace or a call trace, is a series of functions that show the flow of code.

2. **Profiling Stack Traces**

   Timed sampling of stack traces can be tens or hundreds of lines long each. To make this volume of data easier to study, **BCC** profiler tool summarize stack traces in a different way, showing a count for each unique stack trace.

   and **perf**(1) profiler summarizes stack samples as a call tree, and shows percentage for each path. 

   ------

   

## Event Source

1. **Kprobes**

   **Kprobes** provide kernel dynamic instrumentation

   **Kprobes** can create instrumentation events for any kernel function, and it can instrument instructions within functions. It can do this live, in production environment, without needing to either reboot the system to run the kernel in any special mode.
   we can instrument any of the tens of thousands of kernel functions in linux to create new custom metrics as needed.

   The kprobes technology also has an interface called **kretprobes** for instrumenting when functions return, and their return values. when kprobes and kretprobes instrument the same function, timestamps can be recorded to calculate the duration of a function, which can be an important metric for performance analysis.

2. **How Kprobes Work**