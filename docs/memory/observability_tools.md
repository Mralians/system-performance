## Memory observability tools

1. ### vmstat

   ```bash
   vmstat -Sm 1 # to megabytes
   vmstat -a 1  # print a breakdown of inactive and active from page cache
   vmstat -w 1  # wide output
   ```

2. ### PSI (linux pressure stall information) added in linux 4.20

   ```bash
   cat /proc/pressure/memory
   #some avg10=2.84 avg60=1.23 avg300=0.32 total=1468344
   #full avg10=1.85 avg60=0.66 avg300=0.16 total=702578
   ```

   PSI statistics are also tracked per cgroup2

3. ### swapon

   ```bash
   swapon
   #NAME           TYPE       SIZE USED PRIO
   #/dev/nvme0n1p7 partition 15.6G   0B   -2
   ```

4. ### sar (system activity reporter)

   - **-B:** paging statistics

     |  Statistics   |                         Description                          |   Unit   |
     | :-----------: | :----------------------------------------------------------: | :------: |
     |   pgpgin/s    |                           Page-ins                           | Kbytes/s |
     |   pgpgout/s   |                          Page-outs                           | Kbytes/s |
     |   faults/s    |                 Both major and minor faults                  | Count/s  |
     |   majflt/s    |                         Major faults                         | Count/s  |
     |   pgfree/s    |                   Pages added to free list                   | Count/s  |
     |   pgscank/s   |     Pages scanned by background page-out daemon(kswapd)      | Count/s  |
     | **pgscand/s** |                      Direct page scans                       | Count/s  |
     |   pgsteal/s   |                 Page and swap cache reclaims                 | Count/s  |
     |  **%vmeff**   | Ratio of page steal/page scan,which shows page reclaim efficiency<br />The %vmeff metric is a useful measure of page reclaims efficiency. High means pages are successfully stolen from the inactive list(healthy); low means the system is struggling. the man page describes near 100% as high, and less than 30% as low. | Percent  |

   - **-H:** Huge pages statistics

     | Statistics |               Description                |  Unit   |
     | :--------: | :--------------------------------------: | :-----: |
     | hbhugefree | Free huge pages memory (large page size) | Kbytes  |
     | hbhugeused |          Used huge pages memory          | Kbytes  |
     | %hugeused  |             Huge page usage              | Percent |

   - **-r:** Memory utilization

     | Statistics |                         Description                          |  Unit   |
     | :--------: | :----------------------------------------------------------: | :-----: |
     | kbmemfree  |               Free memory (completely unused)                | Kbytes  |
     |  kbavail   | Available memory,including pages that can be readily freed from the page cache | Kbytes  |
     | kbmemused  |              Used memory (excluding the kernel)              | Kbytes  |
     |  %memused  |                         Memory usage                         | Percent |
     | kbbuffers  |                      Buffer cache size                       | Kbytes  |
     |  kbcached  |                       Page cache size                        | Kbytes  |
     |  kbcommit  | Main memory committed: an estimate of the amount needed to serve the current workload | Kbytes  |
     |  %commit   |     Main memory committed for current workload, estimate     | Percent |
     |  kbactive  |                   Active list memory size                    | Kbytes  |
     |  kbinact   |                  Inactive list memory size                   | Kbytes  |
     |  kbdirtyw  |            Modified memory to be written to disk             | Kbytes  |

   - **-r ALL**

     | Statistics |         Description          |  Unit  |
     | :--------: | :--------------------------: | :----: |
     |  kbanonpg  |   Process anonymous memory   | Kbytes |
     |   kbslab   |    Kernel slab cache size    | Kbytes |
     |  kbkstack  |   Kernel stack space size    | Kbytes |
     |  kbpgtbl   | Lowest-level page table size | Kbytes |
     | kbvmuused  |  Used virtual address space  | Kbytes |

   - **-S:** Swap space statistics

     | Statistics |                         Description                          |  Unit   |
     | :--------: | :----------------------------------------------------------: | :-----: |
     | kbswapfree |                       Free swap space                        | Kbytes  |
     | kbswapused |                       Used swap space                        | Kbytes  |
     |  %swpused  |                       Used swap space                        | Percent |
     |  kbswpcad  | Cached swap space: this resides in both main memory and the swap device and so can be paged out without disk  I/O | Kbytes  |
     |  %swpcad   |            Ratio of cached swap versus used swap             | Percent |

   - **-W:** Swapping statistics

     | Statistics |         Description          |  Unit   |
     | :--------: | :--------------------------: | :-----: |
     |  pswpin/s  |  Page-ins(Linux "swap-ins")  | Pages/s |
     | pswpout/s  | Page-outs(Linux "swap-outs") | Pages/s |

5. ### slabtop: prints kernel slab cache usage from the slab allocator

   ```bash
   slabtop -sc # sort by cache size
   ```

6. ### numastat

7. ### ps

   ```bash
   ps -oe pid,pmem,vsz,rss,comm # using the SVR4-style -o option
   ```

8. ### top

   ```bash
   top -o %MEM # sort by memory usage
   ```

9. ### pmap

   ```bash
   pmap -x  5187
   pmap -X  5187  # more details
   pmap -XX 5187
   ```

10. ### perf

    - One-Liners

      ```bash
      # Sample page faults (RSS growth) with stack traces system wide
      perf record -e page-faults -a -g
      # Record all page faults with stack traces for PID 1843, for 60 seconds
      perf record -e page-faults -c 1 -p 1822 -g -- sleep 60
      # Record heap growth via brk(2)
      perf record -e syscalls:sys_enter_brk -a -g
      # Record page migrations on NUMA systems
      perf record -e migrate:mm_migrate_pages -a
      # Count all kmem events, printing a report every second
      perf stat -e 'kmem:*' -a -I 1000
      # Count all vmscan events, printing a report every second
      perf stat -e 'vmscan:*' -a -I 1000
      # Count memory compaction events, printing a report every second
      perf stat -e 'compaction:*' -a -I 1000
      # Trace kswapd wakeup events with stack traces
      perf record -e vmscan:mm_vmscan_wakeup_kswapd  -ag
      # Profile memory access for the given command
      perf mem record command
      # Summarize a memory profile
      perf mem report
      # Print all events
      perf script --header
      ```

      How to generate flame graph?

      ```bash
      # perf record -e page-faults -a -g -- sleep 60
      # perf script --header > out.stacks
      $ git clone https://github.com/brendangregg/FlameGraph; cd FlameGraph
      $ ./stackcollapse-perf.pl < ../out.stacks | ./flamegraph.pl --hash \
      --bgcolor=green --count=pages --title="Page Fault Flame Graph" > out.svg
      ```

      

11. ### drsnoop

    is a BCC tool for tracing the direct reclaim approach to freeing memory, showing the process affected and the latency: the time taken for the reclaim.

    It can be used to quantify the application performance impact of a memory-constrained system.

    ```bash
    drsnoop -T
    ```

    

12. ### wss(working set size)

    this tools works by resetting the PTE accessed bit for every page in a process, pausing for an interval, and then checking the bits to see which have been set.

    WARNINGS: this tool uses `/proc/PID/clear_refs` and `/proc/PID/smaps`, which can cause slightly higher application latency (e.g, **10%**) while the kernel walks page structures. For large processes (> 100Gbytes), this duration of higher latency can last over one second, during which this tool is consuming system CPU time.

    Keep these overheads in mind. this tool also resets the referenced flag, which might confuse the kernel as to which pages to reclaim, especially if swapping is active.

13. ### bpftrace

    ```bash
    # Show libc malloc() request bytes by user stack for PID 181 as a power-of-2 histogram (high overhead)
    bpftrace -e 'uprobe:/lib/x86_64-linux-gnu/libc.so.6:malloc /pid == 181/ { @[ustack] = hist(arg0); }'
    
    # Sum kernel kmem cache allocation bytes by kernel stack trace
    bpftrace -e 't:kmem:kmem_cache_alloc {@bytes[kstack] = sum(args->bytes_alloc);}'
    
    # Count process heap expansion (brk(2)) by code path
    bpftrace -e 'tracepoin:syscalls:sys_enter_brk {@[ustack,comm] = count()}'
    
    # Count page faults by process
    # Tracing page faults shows when a process grows in memory size.
    bpftrace -e 'software:page-fault:1 { @[comm, pid] = count(); }'
    
    # Count user page faults by user-level stack trace
    bpftrace -e 't:exceptions:page_fault_user {@[ustack, comm] = count(); }' > out.stacks
    git clone https://github.com/brendangregg/FlameGraph; cd FlameGraph ./stackcollapse-bpftrace.pl < ../out.stacks | ./flamegraph.pl --hash \
    --bgcolor=green --count=pages --title="Page Fault Flame Graph" > out.svg
    
    # Count vmscan operations by tracepoint
    bpftrace -e 'tracepoint:vmscan:* {@[probe] = count(); }'
    
    # Count swapins by process
    bpftrace -e 'kprobe:swap_readpage {@[comm,pid] = count()}'
    
    # Count page migration
    bpftrace -e 'tracepoint:migration:mm_migrate_pages {@ = count(); }'
    
    # Trace compaction events
    bpftrace -e 't:compaction:mm_compaction_begin { time(); }'
    
    # List USDT probes in libc
    bpftrace -e 'usdt:/lib/x86_64-linux-gnu/libc.so.6:*'
    
    # List kernel kmem tracepoints:
    bpftrace -l 't:kmem:*'
    
    # List all memory subsystem (mm) tracepoints
    bpftracce -l 't:*:mm_*'
    bpftrace -l 'tracepoint:kmem:*'
    
    # Listing USDT probes for libc on ubuntu, if the tracepoints and USDT probes are insufficient, consider using dynamic instrumentation with kprobes and uprobes.
    bpftrace -l 'usdt:/lib/x86_64-linux-gnu/libc.so.6'
    ```

    Since memory events can be very frequent, instrumenting then can consume significant overhead. malloc(3) functions form user space can be called millions of times per second, and with the current uprobes overhead.

    tracing them can slow a target two-fold or more, use caution and find ways to reduce this overhead, such as using maps to summarize statistics instead of printing per-event details, and tracing the fewest possible events.

14. **pmcarch:** CPU cycle usage including LLC misses

15. **tlbstat:** Summarize TLB cycles

16. **free:** Cache capacity statistics

17. **cachestat:** Page cache statistics

18. **oomkill:** shows extra info on OOM kill events

19. **memleak:** Shows possible memory leak code paths

20. **mmapsnoop:** Trace mmap(2) calls system-wide

21. **brkstack:** Show brk() calls with user stack traces

22. **shmsnoop:** Traces shared memory calls with details

23. **faults:** Show page faults, by user stack trace

24. **ffaults:** Show page faults, by filename

25. **vmscan:** Measures VM  scanner shrink and reclaim times

26. **swapin:** Shows swap-ins by process

27. **hfaults:** Shows huge page faults, by process

28. **dmesg:** Check for "Out of memory" message from OOM killer.

29. **dmidecode:** Show BIOS information for memory bank.

30. **tiptop:** A version of top(1) that displays PMC statistics by process.

31. **valgrind:** A performance analysis suite.

32. **iostat:** if the swap device is a physical disk or slice, device I/O may be observable using iostat(1).

33. `/proc/zoneinfo`: Statistics for memory zone.

34. `/proc/buddyinfo:` Statistics for the kernel buddy allocator for pages.

35. `/proc/pagetypeinfo:` Kernel free memory page statistics; can be used to help debug issues of kernel memory fragmentation.

36. `/proc/devices/system/node/node*/numastat`: statistics for NUMA nodes.

37. **SysRq m:** Magic SysRq has an "m" key to dump memory info to the console.

    ```bash
    echo m > /proc/sysrq-trigger
    dmesg
    ```

    This can be useful if the system has locked up, as it may still be possible to request this information using the SysRq key sequence on the console keyboard, if available.