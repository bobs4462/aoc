/// --- Day 4: The Ideal Stocking Stuffer ---
use crate::solver::{Solution, Solver};
use std::sync::mpsc;
use std::thread;
//
use num_cpus;

pub struct D4;

impl Solver for D4 {
    /// Validate that every character in the input is ASCII alphanumeric
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        for (i, &c) in input.iter().enumerate() {
            if !c.is_ascii_alphanumeric() {
                return Err(format!("The character at postion {} is invalid: {}", i, c,));
            }
        }
        Ok(())
    }
    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        self.find_num(data, 0xfff)
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        self.find_num(data, 0xff)
    }
}

impl D4 {
    fn find_num(&self, data: Vec<u8>, compare_to: u32) -> Solution {
        let cores = num_cpus::get() - 1;
        let (tx, rx) = mpsc::channel::<usize>();
        for c in 0..cores {
            let tx = tx.clone();
            let original = data.clone();
            thread::spawn(move || {
                for i in (c..usize::MAX).step_by(cores) {
                    let mut test = original.clone();
                    test.extend(i.to_string().as_bytes());
                    let first = md5(test)[0].to_be();
                    if first <= compare_to {
                        match tx.send(i) {
                            _ => break,
                        }
                    }
                }
            });
        }
        if let Ok(num) = rx.recv() {
            return Solution::new("The number to mine bitcoin is:", num.to_string());
        } else {
            panic!("Channel has hang up unexpectedly");
        }
    }
}

#[allow(dead_code)]
fn digest(array: [u32; 4]) -> String {
    let bytes: &[u8] =
        unsafe { std::slice::from_raw_parts((&array) as *const u32 as *const u8, 16) };
    let mut hexdigest = String::new();
    for b in bytes.iter() {
        hexdigest += &format!("{:02X}", b);
    }
    hexdigest
}

const CHUNK: usize = 0x40; // the size of the chunk in bytes 64
/// 4 rounds with 16 operations in each totalling 64. Each item in S is bitwise shift amount
static S: [u32; CHUNK] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

/// 4 rounds with 16 operations in each totalling 64. Each item in K is used to compute the value
/// to shift on each operation
static K: [u32; CHUNK] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];
static PADDING: [u8; CHUNK] = [
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

fn md5(mut data: Vec<u8>) -> [u32; 4] {
    let mut a0: u32 = 0x67452301; // A
    let mut b0: u32 = 0xefcdab89; // B
    let mut c0: u32 = 0x98badcfe; // C
    let mut d0: u32 = 0x10325476; // D

    // original data length in bits
    let original_len = (data.len() * 8) as u64;
    // pad the data with 1, without any checks
    data.push(PADDING[0]);
    // Find the length of data in bytes modulo 64, find the difference with 56 = 448 bits, the
    // difference can be negative or positive, if it's positive modulo magic will leave it as is,
    // but if it's negative it will become a positive number 64 - abs(difference), which is the
    // amount of bytes to pad the original data with
    let padding =
        (((56 - (data.len() % CHUNK) as isize) + CHUNK as isize) % CHUNK as isize) as usize;
    // append the padding amount of bytes so that the total length of data becomes congruent to 56
    // modulo 64
    data.extend(PADDING[1..=padding].iter());
    // append 8 bytes which encode little endian representation of original length of data
    data.append(&mut original_len.to_le_bytes().to_vec());

    let chunks = data.chunks(CHUNK);
    for chnk in chunks {
        let m: &[u32] = unsafe { std::slice::from_raw_parts(chnk.as_ptr() as *const u32, 16) };
        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;
        for i in 0..CHUNK {
            let mut f: u32 = 0;
            let mut g: usize = 0;
            if i <= 15 {
                f = (b & c) | ((!b) & d);
                g = i;
            } else if i >= 16 && i <= 31 {
                f = (d & b) | ((!d) & c);
                g = (5 * i + 1) % 16;
            } else if i >= 32 && i <= 47 {
                f = b ^ c ^ d;
                g = (3 * i + 5) % 16;
            } else if i >= 48 && i <= 63 {
                f = c ^ (b | (!d));
                g = (7 * i) % 16;
            }
            f = f.wrapping_add(a).wrapping_add(K[i]).wrapping_add(m[g]); // m[g] must be a 32-bits block
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f << S[i] | f >> (32 - S[i]));
        }
        // add this chunk's hash to result so far:
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }
    [a0, b0, c0, d0]
}

#[cfg(test)]
mod tests {
    use crate::solver::Solver;
    #[test]
    fn test_md5() {
        let data = "Lorem Ipsum - это текст-\"рыба\", часто используемый в печати и вэб-дизайне. Lorem Ipsum является стандартной \"рыбой\" для текстов на латинице с начала XVI века. В то время некий безымянный печатник создал большую коллекцию размеров и форм шрифтов, используя Lorem Ipsum для распечатки образцов. Lorem Ipsum не только успешно пережил без заметных изменений пять веков, но и перешагнул в электронный дизайн. Его популяризации в новое время послужили публикация листов Letraset с образцами Lorem Ipsum в 60-х годах и, в более недавнее время, программы электронной вёрстки типа Aldus PageMaker, в шаблонах которых используется Lorem Ipsum.".as_bytes().to_vec();
        let res = super::md5(data);
        // assert_eq!(res, hash);
        let hash = "4F15CB2065FB348EA4DD6DD1F05153F3";
        assert_eq!(super::digest(res), hash);
    }

    #[test]
    fn test_part_one() {
        let data = b"abcdef".to_vec();
        let solver = super::D4 {};
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "609043");
    }
    #[test]
    fn test_part_two() {
        let data = b"ckczppom".to_vec();
        let solver = super::D4 {};
        let res = solver.solve_part_two(data);
        assert_eq!(res.value, "3938038");
    }
}
