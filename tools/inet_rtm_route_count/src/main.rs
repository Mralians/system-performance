// route change event watcher

use bcc;
use byteorder::{NativeEndian, ReadBytesExt};

use anyhow::{Context, Result};
use core::sync::atomic::{AtomicBool, Ordering};
use std::io::Cursor;
use std::sync::Arc;
use std::thread;
use std::time;

fn do_main(runnable: Arc<AtomicBool>) -> Result<u64, bcc::BccError> {
    let bpf_text = include_str!("bpf.c");
    let mut module = bcc::BPF::new(bpf_text).unwrap();
    let mut value = 0;
    bcc::Kprobe::new()
        .handler("do_count")
        .function("inet_rtm_newroute") //TODO: add 'inet_rtm_delroute' kernel function
        .attach(&mut module)?;
    let table = module.table("counts")?;
    while runnable.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::new(1, 0));
        for entry in &table {
            value = Cursor::new(entry.value).read_u64::<NativeEndian>().unwrap();
        }
    }
    Ok(value)
}

fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let runnable = Arc::new(AtomicBool::new(true));
    let r = runnable.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .with_context(|| format!("failed to set handler"))?;

    thread::spawn(|| {
        thread::sleep(time::Duration::new(2, 0));
        println!("Tracing... Ctrl-C to end.");
    }); // FIXME: find another way!
    let value =
        do_main(runnable).with_context(|| format!("failed to run bpftrace:with_context"))?;
    println!(" inet_rtm_newroute => {value}");
    Ok(())
}
