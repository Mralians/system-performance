# Inspecting the Loaded Program

1. The bpftool utility can list all the programs that are loaded into the kernel. If you try this yourself you'll probably see several preexisting eBPF programs in this output.
   ```bash
   $ bpftool prog list
   
   31: xdp name hello tag d35b94b4c0c10efb gpl
   loaded_at 2022-08-02T17:39:47+0000 uid 0
   xlated 96B jited 148B memlock 4096B map_ids 165,166
   btf_id 254
   ```

2. The program has been assigned the ID 540. this identity is a number assigned to each program as it's loaded.

3. Knowing the ID, you can ask bpftool to show more information about this program.
   ```bash
   $ bpftool prog show id 31 --pretty
   {
   # The program's ID is 31.
       "id": 31,
   # The type field tells us this program can be attached to a network interface using
   # XDP event.
       "type": "xdp",
   # The name of the program iis hello.
       "name": "hello",
       "tag": "4ae0216d65106432",
   # The program is defiend with GPL-compatible license.
       "gpl_compatible": true,
   # There's a timestamp showing when the program was loaded.
       "loaded_at": 1681314659,
   # User ID 0 (which is root) loaded the program.
       "uid": 0,
   # There are 168 bytes of translated eBPF bytecode in this program.
       "bytes_xlated": 168,
   # This program has been JIT-compiled, and the compilation resulted in 105 bytes of
   # machine code.
       "jited": true,
       "bytes_jited": 105,
       "bytes_memlock": 4096,
    # This program refers to BPF maps with ID 3.
       "map_ids": [3
       ],
       "btf_id": 177
   }
   ```

4. eBPF programs can be loaded into the kernel without being pinned to a file location, but it's not optional for bpftool, which always has to pin the programs it loads.

## The BPF program Tag

5. The ID can vary every time you load or unload the program, but the tag will remain the same.

6. The bpftool utility accepts references to a BPF program by **ID**, **name**, **tag** or **pinned path**, so in the example here, all of the following would give the same output.
   ```bash
   $ bpftool prog show id 31
   $ bpftool prog show name hello
   $ bpftool prog show tag 4ae0216d65106432
   $ pbftool prog show pinned /sys/fs/bpf/hello
   ```

7. You could have multiple programs with the same name, and even multiple instances of programs with the same tag, but the ID and pinned path with always be unique.

8. The `bytes_xlated` field tells us how many bytes of "translated" eBPF code there are.

9. This is the eBPF bytecode after it has passed through the verifier (and possibly been modified by the kernel for reasons).
   ```bash
   $ bpftool prog dump xlated id 31
   ```

## The JIT-Compiled Machine Code

10. The translated bytecode is pretty low level, but it's not quite machine code yet. eBPF uses a JIT compiler to convert eBPF bytecode to machine code that runs natively on the target CPU.

11. The `bytes_jited` field show that after this conversion the program is 108 bytes long.

12. For higher performance, eBPF programs are generally JIT-Compiled. The alternative is to interpret thee eBPF bytecode at run-time.

13. The bpftool utility can generate a dump of this JITed code in assembly language.
    ```bash
    $bpftool prog dump jited id 31
    ```