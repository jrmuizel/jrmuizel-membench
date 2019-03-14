use std::time::Instant;

fn main() {
    let size = 3840*2160*4*60;
    let src: Vec<u8> = vec![1; size];
    let mut dst: Vec<u8> = vec![0; size];
    loop {
        let begin = std::time::Instant::now();
        dst.copy_from_slice(&src[..]);
        let end = std::time::Instant::now();
        println!("time: {} ms", (end - begin).as_nanos() as f32 / (60. * 1000. * 1000.));
    }
}
