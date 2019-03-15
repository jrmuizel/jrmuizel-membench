use std::time::Instant;
use num_cpus;

fn mem_bench(count: u32) {
    let size = 3840*2160*4*count as usize;
    let src: Vec<u8> = vec![1; size];
    let mut dst: Vec<u8> = vec![0; size];
    loop {
        let begin = std::time::Instant::now();
        dst.copy_from_slice(&src[..]);
        let end = std::time::Instant::now();
        println!("time: {} ms", (end - begin).as_nanos() as f32 / (count as f32 * 1000. * 1000.));
    }
}

fn work_loop(count: i32) {
    for i in 0..count {
        std::time::Instant::now();
    }
}


fn cpu_bench(core: usize, count: u32) {
    loop {
        let begin = std::time::Instant::now();
        work_loop(10_i32.pow(count));
        let end = std::time::Instant::now();
        println!("{}: time: {} ms", core, (end - begin).as_nanos() as f32 / (60. * 1000. * 1000.));
    }
}

enum Kind {
    Cpu,
    CpuAll,
    Mem
}

fn main() {
    let opt = std::env::args().nth(1);
    let kind = if let Some(kind) = opt {
        match kind.as_ref() {
            "cpu" => Kind::Cpu,
            "cpuall" => Kind::CpuAll,
            _ => Kind::Mem
        }
    } else {
        Kind::Mem
    };

    let amount = std::env::args().nth(2).and_then(|x| x.parse().ok());

    match kind {
        Kind::Cpu => cpu_bench(0, amount.unwrap_or(8)),
        Kind::Mem => mem_bench(amount.unwrap_or(60)),
        Kind::CpuAll => {
            let count = num_cpus::get();
            let mut join = None;
            for i in 0..count {
                join = Some(std::thread::spawn(move || {
                    cpu_bench(i, amount.unwrap_or(8))
                }));
            }
            join.unwrap().join();
        },

    }
}
