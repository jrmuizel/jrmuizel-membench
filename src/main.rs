use std::time::Instant;

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
    for i in 0..10000000 {
        std::time::Instant::now();
    }
}


fn cpu_bench(count: u32) {
    loop {
        let begin = std::time::Instant::now();
        work_loop(10_i32.pow(count));
        let end = std::time::Instant::now();
        println!("time: {} ms", (end - begin).as_nanos() as f32 / (60. * 1000. * 1000.));
    }
}


fn main() {
    let opt = std::env::args().nth(1);
    let cpu = if let Some(opt) = opt {
        opt == "cpu"
    } else {
        false
    };

    let amount = std::env::args().nth(2).and_then(|x| x.parse().ok());

    if cpu {
        cpu_bench(amount.unwrap_or(6));
    } else {
        mem_bench(amount.unwrap_or(60));
    }
}
