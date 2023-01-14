use std::io;

fn main() {
    let mut line1 = String::new();
    let _ = io::stdin().read_line(&mut line1);
    let mut line2 = String::new();
    let _ = io::stdin().read_line(&mut line2);
    let mut line3 = String::new();
    let _ = io::stdin().read_line(&mut line3);
    let mut line4 = String::new();
    let _ = io::stdin().read_line(&mut line4);
    let a: u32 = line1.trim().parse().unwrap();
    let b: u32 = line2.trim().parse().unwrap();
    let c: u32 = line3.trim().parse().unwrap();
    let x: u32 = line4.trim().parse().unwrap();

    let mut cnt = 0;

    for i in 0..a + 1 {
        for j in 0..b + 1 {
            for k in 0..c + 1 {
                if i * 500 + j * 100 + k * 50 == x {
                    // println!("{} {} {}", i, j, k);
                    cnt = cnt + 1;
                }
            }
        }
    }

    println!("{}", cnt);
}
