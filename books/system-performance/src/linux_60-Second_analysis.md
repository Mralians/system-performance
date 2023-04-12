## Linux 60-Second Analysis

1. uptime

   this is a quick way to view the load average, which indicate the number of tasks(process) wanting to run. On Linux system, these numberts include process wanting to run on the CPUs, **as well as processes blocked in uninterruptible  I/O** (usually disk I/O).

2. dmesg | tail

3. vmstat 1

   ```bash
   --procs-- -----------------------memory---------------------- ---swap-- -----io---- -system-- --------cpu--------
      r    b         swpd         free         buff        cache   si   so    bi    bo   in   cs  us  sy  id  wa  st
      2    0            0      4390360        93524      1843900    0    0   171    25  103  252   2   1  97   0   0
      0    0            0      4384880        93524      1843900    0    0     0     0  544  975   1   0  99   0   0
      0    0            0      4386152        93524      1843964    0    0     0    16  932 2595   1   0  99   0   0
   
   ```

   **r:** the number of processes running on CPU and waiting for a turn. this provide a better signal than load averages for determining CPU saturation, as it does not include I/O. **To interpret: an "r" value greater than CPU count indicates saturation**

   **si and so:** Swap-ins and swap-outs. if these are non-zero, you're out of memory.

   **us, sy, id, wa, and st:** These are breakdowns of CPU time, on average, across all CPUs. they are user time, system time(kernel), idle, wait I/O, and stolen time(the guest's own isolated driver domain).

   

4. mpstat -P ALL 1

   This command prints per-CPU time broken down into states.

   **high %iowait time**, which can be explored with disk I/O tools, and **high %sys time**, which an be explored with syscall and kernel tracing, as well as CPU profiling.

5. pidstat 1

   shows CPU usage per process.

6. iostat -xz 1

   This tool shows storage device I/O metrics. The output columns for each disk device have line-wrapped here.

   **await**: The average time for the I/O in milliseconds. This is time that the application suffers, as it include both time queued and time being serviced. Larger-than-expected average times can be an indicator of device saturation or device problem.

7. free -m

8. sar -n DEV 1 

   The sar tool has many modes for different groups of metrics. Here I'm using it to look at network device metrics. Check interface throughput rxKB/s and txKB/s to see if any limit may have been reached.

9. sar -n TCP,ETCP 1

   TCP metrics and TCP error.

   **active/s:** 			Number of locally initiated TCP connections per second(e.g., via connect())

   **passive/s:** 		 Number of remotely initiated TCP connections per second (e.g, via accept())	

   **retrans/s:** 		  Number of TCP retransmits per second.

10. top