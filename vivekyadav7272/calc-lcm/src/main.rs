fn LCM(a: u32, b: u32) -> u32 {
    if b == 1 || a == b {
        a
    } else {
        let k = if b-a < a { LCM(b-a, a) } else { LCM(a, b-a) };
        let m = k/(b-a);
        m*a + k
    }
}

fn main() {
    let egs = [(4,6), (12,15), (2,8), (3,7)];
    for eg in egs {
        println!("LCM{:?} = {}", eg, LCM(eg.0, eg.1));
    }
}