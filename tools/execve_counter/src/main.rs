use bcc::{Kprobe, BPF};
use byteorder::{NativeEndian, ReadBytesExt};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

mod pwd;
use pwd::Passwd;

fn main() -> Result<(), Box<dyn Error>> {
    let code = include_str!("./bpf.c");
    let mut b = BPF::new(code)?;
    let syscall = b.get_syscall_fnname("execve");
    Kprobe::new()
        .function(&syscall)
        .handler("execve_counter")
        .attach(&mut b)?;
    loop {
        // For simplicity, i used "loop" instead of "ctrlc" for now
        println!("{:<-10} {:<-10}", "UID", "PID");
        let table = b.table("counter_table")?;
        for e in table {
            let uid = std::io::Cursor::new(&e.key).read_u64::<NativeEndian>()?;
            let count = std::io::Cursor::new(&e.value).read_u64::<NativeEndian>()?;
            let pwd = Passwd::getpwuid(uid as u32).unwrap();
            if count > 0 {
                println!("{:<-10} {:<-10}", pwd.username(), count);
            }
        }
        sleep(Duration::new(1, 0));
        print!("\x1B[2J\x1B[1;1H");
    }
}
