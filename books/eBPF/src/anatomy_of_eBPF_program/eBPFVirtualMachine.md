# The eBPF Virtual Machine

1.  The eBPF virtual machine takes in a program in the form of eBPF **bytecode** instructions, and these have to be converted to native machine instructions that run on the CPU.
2.  In early implementations of the eBPF, the bytecode instructions were interpreted within the kernel, every time an eBPF program runs, the kernel examines the instructions and convert them into machine code.
3.  This method replaced by `JIT` compilation for performance reasons and to avoid the possibility of some `Spectre-related` vulnerabilities in the eBPF interpreter.
4.  Compilation means the conversion to native machine instructions happens just once, when the program is loaded into the kernel. 
5.   eBPF bytecode consists of a set of instructions, and those instructions act on (virtual) eBPF registers.
6.  The eBPF virtual machines uses **10** general-purpose registers, numbered 0 to 9.
7.  Register 10 is used as stack frame pointer and can only be read, but not written.
8.  As a BPF program is executed, values get stored in these registers to keep track of state.
9.  eBPF registers in the eBPF virtual machine are implemented in software. You can see them enumerated from `BPF_REG_0` to `BPF_REG_10`.
10.  The context argument to an eBPF program is loaded into **Register 1** before its execution begins. The return value from the function is stored in **Register 0**.
11.  Before calling a function from eBPF code, the arguments to that function are placed in Register 1 through Register 5.
12.  `linux/bpf.h` defines a structure called `bpf_insn`, which represents a BPF instruction.

```c
struct bpf_insn {
__u8 code;			/* opcode */
__u8 dst_reg:4;			/* dest register */
__u8 src_reg:4;			/* source register */
__s16 off;			/* signed offset */
__s32 imm;			/* signed immediate constant */
};
```

13. `__u8 code`: Each instruction has an opcode, which defines what operation the instruction is to perform: for example, adding a value to the contents of a register, or jumping to a different instruction in the program.
14. `__u8 dst_reg:4`: Different operations might involve up to two registers.
15. `__s16 off` and `__s32 imm`: Depending on the operation, there might be an offset value and/or an "immediate" integer value.
16. `bpf_insn` structure is 64 bit  long.
17. eBPF program when loaded into kernel, the bytecode of an eBPF program is represented by a series of these `bpf_insn` structures. The verifier performs several checks on this information to ensure that the code is safe to run.
18. Opcode categories:
    - Loading a value into a register
    - Storing a value from a register into memory
    - Performing arithmetic operation such as adding a value to the contents of a register
    - Jumping to a different instruction if a particular condition is satisfied.