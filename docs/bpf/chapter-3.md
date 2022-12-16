# Chapter 3

#### Performance Analysis

------

1. ### Goals

   1. The goal of performance analysis are to improve end-user performance and to reduce operating cost.
   2. **Latency:** How long to accomplish a request or operation, typically measured in milliseconds.
   3. **Rate:** An operation or request rate per second.
   4. **Throughput:** typically data movement in bits or bytes per second.
   5. **Utilization:** How busy a resource is over time as a percentage.
   6. **Cost:** the price/performance ratio

   

2. ### Performance Methodologies

   1. **Workload Characterization**

      1. The aim of workload characterization is to understand the applied workload.

      2. eliminating unnecessary work.

      3. suggested steps for performing workload characterization:

         1. Who is causing the load (e.g., PID, process name, UID, ip address)?
         2. Why is load called (code path, stack trace, flame graph)?
         3. What is the load (IOPS, throughput, type)?
         4. How is the load changing over time (per-interval summaries)?

         ```bash
         vfsstat
         TIME         READ/s  WRITE/s  FSYNC/s   OPEN/s CREATE/s
         22:58:25:      1391       81        0       14        0
         22:58:26:        32       17        0       10        0
         22:58:27:        61       31        0       20        0
         22:58:28:        35       19        0       11        0
         22:58:29:        53       36        0       12        0
         22:58:30:        77       65        0       11        0
         22:58:31:        45       31        0       10        0
         22:58:32:       123       89        0       29        0
         
         # this shows details of the workload applied at the virual file system(vfs) level
         # and answers step 3 by providing the types and operation rates, the step 4 by 
         # providing the per-interval summary over time.
         
         bpftrace -e 'kprobe:vfs_read  { @[comm] = count(); }'
         bpftrace -e 'kprobs:vfs_write {@[comm] = count(); }'
         ```

         

      4. 
