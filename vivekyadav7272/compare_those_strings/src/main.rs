use core::num;
use std::io::BufRead;
mod solution2;

fn solution1(str1: &[u8], str2: &[u8]) -> u32 {
    assert_eq!(str1.len(), str2.len());

    let start = 0;
    let i = 0usize;

    while i < str1.len() {
        if str1[i] >= str2[i] {
            start = i + 1;
        }

        i += 1;
    }

    (str1.len() - i) as u32
}

fn solution2(str1: &[u8], str2: &[u8]) -> usize {
    assert_eq!(str1.len(), str2.len());

    let mut i = str1.len() - 1;
    
    while i >= 0 {
        if str1[i] >= str2[i] {
            break;
        }

        i -= 1;
    }

    str1.len() - i
}

fn stdin_locked() -> std::io::StdinLock<'static> {
    let stdin = std::io::stdin();
    stdin.lock()
}

fn parse_and_clean<T: FromStr>(buf: &mut String) -> Result<T, ()> {
    let ret = buf.parse::<T>()?;


}

fn take_input() -> Vec<String> {
    let stdin = stdin_locked();
    let mut input_buf = String::new();
    stdin.read_line(&mut input_buf).unwrap();
    let num_trials = input_buf.parse::<u32>().unwrap();
    
    for _ in 0..num_trials {
        stdin
    }
}

fn main() {
    use std::io::Read;

    let stdin = take_input();
}
