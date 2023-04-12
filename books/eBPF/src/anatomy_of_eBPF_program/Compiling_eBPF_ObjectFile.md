# Compiling an eBPF Object File



1. eBPF source code needs to be compiled into the machine instructions that the eBPF virtual machine can understand.

2. The Clang compiler from the LLVM project will do this if you specify `--target bpf`
   ```makefile
   hello.bpf.o: %.o: %.c
   clang \
   -target bpf \
   -I/usr/include/$(shell uname -m)-linux-gnu \
   -g \
   -O2 -c $< -o $@
   ```

​		this generates an object file called `hello.bpf.o` from source code.

## Inspecting an eBPF Object File

```bash
$ llvm-objdump-14 -S ./hello.bpf.o

./hello.bpf.o:	file format elf64-bpf

Disassembly of section xdp:

0000000000000000 <hello>:
; int hello(void *ctx) {
       0:	b7 01 00 00 00 00 00 00	r1 = 0
;   bpf_printk("Hello World %d", counter);
# Instructions are generally 8 bytes long
# The first byte of each line in the opcode that tells the kernel what operation to perform
       1:	73 1a fe ff 00 00 00 00	*(u8 *)(r10 - 2) = r1
       2:	b7 01 00 00 25 64 00 00	r1 = 25637
       3:	6b 1a fc ff 00 00 00 00	*(u16 *)(r10 - 4) = r1
# Line 4: 0xb7 tells us the pseudocode corresponding to this is dst = imm, which can be read
# as "Set the destination to the immediate value."
# The destination is defined by the second byte, 0x01, which means "Register 1".
# Set Register 1 to value 543452274
       4:	b7 01 00 00 72 6c 64 20	r1 = 543452274
       5:	63 1a f8 ff 00 00 00 00	*(u32 *)(r10 - 8) = r1
       6:	18 01 00 00 48 65 6c 6c 00 00 00 00 6f 20 57 6f	r1 = 8022916924116329800 ll
       8:	7b 1a f0 ff 00 00 00 00	*(u64 *)(r10 - 16) = r1
       9:	18 06 00 00 00 00 00 00 00 00 00 00 00 00 00 00	r6 = 0 ll
      11:	61 63 00 00 00 00 00 00	r3 = *(u32 *)(r6 + 0)
      12:	bf a1 00 00 00 00 00 00	r1 = r10
      13:	07 01 00 00 f0 ff ff ff	r1 += -16
;   bpf_printk("Hello World %d", counter);
      14:	b7 02 00 00 0f 00 00 00	r2 = 15
      15:	85 00 00 00 06 00 00 00	call 6
;   counter++;
      16:	61 61 00 00 00 00 00 00	r1 = *(u32 *)(r6 + 0)
      17:	07 01 00 00 01 00 00 00	r1 += 1
      18:	63 16 00 00 00 00 00 00	*(u32 *)(r6 + 0) = r1
;   return XDP_PASS;
      19:	b7 00 00 00 02 00 00 00	r0 = 2
      20:	95 00 00 00 00 00 00 00	exit
```

