fn main() {
    use std::io::Read;

    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut input = input.lines();
    let n = input.next().and_then(|n_str| n_str.parse::<u32>().ok()).unwrap();
}