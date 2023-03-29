use bcc::{perf_event::PerfMapBuilder, Kprobe, BPF};
use std::env;
use std::error::Error;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;
#[repr(C)]
struct data_t {
    pid: u32,
    uid: u32,
    command: [u8; 16],
}
fn main() -> Result<(), Box<dyn Error>> {
    let user_syscall = env::var("SYSCALL").unwrap_or("execve".to_owned());
    let code = include_str!("./bpf.c");
    let mut b = BPF::new(code)?;
    let syscall = b.get_syscall_fnname(&user_syscall);
    Kprobe::new()
        .function(&syscall)
        .handler("execve_counter")
        .attach(&mut b)?;
    let table = b.table("output")?;
    let mut perf_map = PerfMapBuilder::new(table, data_callback).build()?;
    println!("{:<-5} {:<-5} {:<-15}", "PID", "UID", "COMMAND");
    loop {
        perf_map.poll(200);
        sleep(Duration::new(2, 0));
    }
}

fn data_callback() -> Box<dyn FnMut(&[u8]) + Send> {
    Box::new(|x| {
        let data = parse_struct(x);
        let command_utf8_string = String::from_utf8(data.command.to_vec())
            .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
            .unwrap();
        println!(
            "{:<-5} {:<-5} {:<-16}",
            data.pid, data.uid, command_utf8_string,
        );
    })
}

fn parse_struct(x: &[u8]) -> data_t {
    unsafe { ptr::read_unaligned(x.as_ptr() as *const data_t) }
}
