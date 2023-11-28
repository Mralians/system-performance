use std::io::Result as ioResult;
use std::{fs::File, io::Read, path::Path};
#[derive(Debug, Default)]
struct System {
    mem_total: u64,
    mem_free: u64,
    mem_available: u64,
    mem_buffers: u64,
    mem_page_cache: u64,
    mem_shmem: u64,
    mem_slab_reclaimable: u64,
    swap_total: u64,
    swap_free: u64,
}

fn get_all_data<T>(path: T, size: usize) -> ioResult<String>
where
    T: AsRef<Path>,
{
    let mut data = String::with_capacity(size);
    File::open(path.as_ref()).and_then(|mut f| f.read_to_string(&mut data))?;
    Ok(data)
}
impl System {
    fn refresh_memory(&mut self) {
        if let Ok(data) = get_all_data("/proc/meminfo", 16_385) {
            for line in data.lines() {
                let mut iter = line.split(':');
                let field = match iter.next() {
                    Some("MemTotal") => &mut self.mem_total,
                    Some("MemFree") => &mut self.mem_free,
                    Some("MemAvailable") => &mut self.mem_available,
                    Some("Buffers") => &mut self.mem_buffers,
                    Some("Cached") => &mut self.mem_page_cache,
                    Some("Shmem") => &mut self.mem_shmem,
                    Some("SReclaimable") => &mut self.mem_slab_reclaimable,
                    Some("SwapTotal") => &mut self.swap_total,
                    Some("SwapFree") => &mut self.swap_free,
                    _ => continue,
                };
                if let Some(str_val) = iter.next().and_then(|s| s.trim_start().split(' ').next()) {
                    if let Ok(value) = u64::from_str_radix(str_val, 10) {
                        *field = value / 1024;
                    }
                }
            }
        }
    }
    fn used_memory(&self) -> u64 {
        self.mem_total - self.mem_available
    }
}
fn main() {
    let mut system = System::default();
    system.refresh_memory();
    println!("{}", system.used_memory());
}
