# Anatomy of an eBPF Program



1. An eBPF program is a set of eBPF bytecode instructions.
2. It,s possible to write eBPF code directly in this bytecode, much as it's possible to program in assembly language.
3. Vast majority of eBPF code is written in C and then compiled to eBPF bytecode. and this bytecode runs in an eBPF virtual machine within the kernel.