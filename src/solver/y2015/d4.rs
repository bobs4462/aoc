use std::{convert::TryInto, intrinsics::transmute};

/// --- Day 4: The Ideal Stocking Stuffer ---
// use crate::solver::{Solution, Solver};
//
// pub(crate) struct D4;
//

fn digest(array: [u32; 4]) -> String {
    let bytes: [u8; 16] = unsafe { transmute(array) };
    let mut hexdigest = String::new();
    for b in bytes.iter() {
        hexdigest += &format!("{:02X}", b);
    }
    hexdigest
}

fn md5(mut data: Vec<u8>) -> [u32; 4] {
    const CHUNK: usize = 0x40; // the size of the chunk in bytes 64
    /// 4 rounds with 16 operations in each totalling 64. Each item in S is bitwise shift amount
    const S: [u32; CHUNK] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];

    /// 4 rounds with 16 operations in each totalling 64. Each item in K is used to compute the value
    /// to shift on each operation
    const K: [u32; CHUNK] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];
    let mut a0: u32 = 0x67452301; // A
    let mut b0: u32 = 0xefcdab89; // B
    let mut c0: u32 = 0x98badcfe; // C
    let mut d0: u32 = 0x10325476; // D

    // original data length in bits
    let original_len = (data.len() * 8) as u64;
    // pad the data with bit 1, and because we cannot operate on bits, add another 7 0s
    data.push(0b10000000);
    // Find the length of data in bytes modulo 64, find the difference with 56 = 448 bits, the
    // difference can be negative or positive, if it's positive modulo magic will leave it as is,
    // but if it's negative it will become a positive number 64 - abs(difference), which is the
    // amount of bytes to pad the original data with
    let padding =
        (((56 - (data.len() % CHUNK) as isize) + CHUNK as isize) % CHUNK as isize) as usize;
    // append the padding amount of bytes so that the total length of data becomes congruent to 56
    // modulo 64
    data.append(&mut vec![0; padding]);
    // append 8 bytes which encode little endian representation of original length of data
    data.append(&mut original_len.to_le_bytes().to_vec());

    let chunks = data.chunks(CHUNK);
    for chnk in chunks {
        let m = chnk
            .chunks(4)
            .map(|c| u32::from_le_bytes(c.try_into().expect("FAILDED TO CONVERT")))
            .collect::<Vec<u32>>();
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
    use std::intrinsics::transmute;
    #[test]
    fn test_md5() {
        let data = "Hello world!".as_bytes().to_vec();
        let hash = [
            0x86, 0xFB, 0x26, 0x9D, 0x19, 0x0D, 0x2C, 0x85, 0xF6, 0xE0, 0x46, 0x8C, 0xEC, 0xA4,
            0x2A, 0x20,
        ];
        let hash = unsafe { transmute::<[u8; 16], [u32; 4]>(hash) };
        let res = super::md5(data);
        assert_eq!(res, hash);
        let hash = "86FB269D190D2C85F6E0468CECA42A20";
        assert_eq!(super::digest(res), hash);
    }
}

// // : All variables are unsigned 32 bit and wrap modulo 2^32 when calculating
// var int s[64], K[64]
// var int i
//
// // s specifies the per-round shift amounts
// s[ 0..15] := { 7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22 }
// s[16..31] := { 5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20 }
// s[32..47] := { 4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23 }
// s[48..63] := { 6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21 }
//
// // Use binary integer part of the sines of integers (Radians) as constants:
// for i from 0 to 63 do
//     K[i] := floor(232 × abs (sin(i + 1)))
// end for
// // (Or just use the following precomputed table):
// K[ 0.. 3] := { 0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee }
// K[ 4.. 7] := { 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501 }
// K[ 8..11] := { 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be }
// K[12..15] := { 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821 }
// K[16..19] := { 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa }
// K[20..23] := { 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8 }
// K[24..27] := { 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed }
// K[28..31] := { 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a }
// K[32..35] := { 0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c }
// K[36..39] := { 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70 }
// K[40..43] := { 0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05 }
// K[44..47] := { 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665 }
// K[48..51] := { 0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039 }
// K[52..55] := { 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1 }
// K[56..59] := { 0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1 }
// K[60..63] := { 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391 }
//
// // Initialize variables:
// var int a0 := 0x67452301   // A
// var int b0 := 0xefcdab89   // B
// var int c0 := 0x98badcfe   // C
// var int d0 := 0x10325476   // D
//
// // Pre-processing: adding a single 1 bit
// append "1" bit to message
//  // Notice: the input bytes are considered as bits strings,
//  //  where the first bit is the most significant bit of the byte.[51]
//
// // Pre-processing: padding with zeros
// append "0" bit until message length in bits ≡ 448 (mod 512)
// append original length in bits mod 264 to message
//
// // Process the message in successive 512-bit chunks:
// for each 512-bit chunk of padded message do
//     break chunk into sixteen 32-bit words M[j], 0 ≤ j ≤ 15
//     // Initialize hash value for this chunk:
//     var int A := a0
//     var int B := b0
//     var int C := c0
//     var int D := d0
//     // Main loop:
//     for i from 0 to 63 do
//         var int F, g
//         if 0 ≤ i ≤ 15 then
//             F := (B and C) or ((not B) and D)
//             g := i
//         else if 16 ≤ i ≤ 31 then
//             F := (D and B) or ((not D) and C)
//             g := (5×i + 1) mod 16
//         else if 32 ≤ i ≤ 47 then
//             F := B xor C xor D
//             g := (3×i + 5) mod 16
//         else if 48 ≤ i ≤ 63 then
//             F := C xor (B or (not D))
//             g := (7×i) mod 16
//         // Be wary of the below definitions of a,b,c,d
//         F := F + A + K[i] + M[g]  // M[g] must be a 32-bits block
//         A := D
//         D := C
//         C := B
//         B := B + leftrotate(F, s[i])
//     end for
//     // Add this chunk's hash to result so far:
//     a0 := a0 + A
//     b0 := b0 + B
//     c0 := c0 + C
//     d0 := d0 + D
// end for
//
// var char digest[16] := a0 append b0 append c0 append d0 // (Output is in little-endian)
//
// // leftrotate function definition
// leftrotate (x, c)
//     return (x << c) binary or (x >> (32-c));
