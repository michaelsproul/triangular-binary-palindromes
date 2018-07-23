fn main() {
    let mut t: u64 = 0;

    for i in 1.. {
        let leading_zeros = vec!['0' as u8; t.trailing_zeros() as usize];
        let bit_str = format!("{}{:0b}", String::from_utf8(leading_zeros).unwrap(), t);

        println!("{} = {}", t, bit_str);
        let n = bit_str.len();
        for j in 0..n / 2 {
            if bit_str.as_bytes()[j] != bit_str.as_bytes()[n - j - 1] {
                panic!("conjecture is false!");
            }
        }

        t += i;
    }
}
