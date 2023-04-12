# Attaching to an Event

1. The program type has to match the type of event it's being attached to.

2. We can use `bpftool` to attach the example eBPF program to the XDP event on on a network interface.
   ```bash
   $ bpftool net attach xdp id 31 dev wlp2s0
   $ bpftool net list
   # The program with ID 31 is attached to the XDP event on the eth0 interface.
   # We can attach eBPF programs to: tc and flow_dissector.
   xdp:
   wlp2s0(3) generic id 31
   
   tc:
   
   flow_dissector:
   $ ip addr show
   # This output shows that wlp2s0 has a JIT-compiled eBPF program, with identify 31,
   # attached to its XDP hook.
   3: wlp2s0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 xdpgeneric/id:31 qdisc noqueue state UP group default qlen 1000
   ```

3. At this point, the hello eBPF program should be producing trace output every time a network packet is received. We can check this out by running `cat /sys/kernel/debug/tracing/trace_pipe` or using `bpftool prog tracelog`.
   ```bash
   <idle>-0
    [003] d.s.. 655370.944105: bpf_trace_printk: Hello World 4531
   <idle>-0
    [003] d.s.. 655370.944587: bpf_trace_printk: Hello World 4532
   <idle>-0
    [003] d.s.. 655370.944896: bpf_trace_printk: Hello World 4533
   ```