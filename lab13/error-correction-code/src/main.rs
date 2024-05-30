use std::cmp::{max, min};
use std::ops::{BitXor, BitXorAssign, Not, Shl};
use rand::Rng;

fn div_xor(mut word: u64, byte: u8) -> u8 {
    let mut k = 63;
    println!("byte = {:#010b}", byte);
    loop {
        if k < 8 {
            break word as u8;
        }
        if word & (1 << k) != 0 {
            // println!("k = {}", k);
            // println!("before: {:#018b}", word);
            let mask = !((1_u64.wrapping_shl(k)) as u64);
            word = word & mask;
            // println!("mid   : {:#018b}", word);
            // println!("x arg : {:#018b}", byte);
            let xor_mask = (byte as u64) << (k - 8);
            // println!("xor m : {:#018b}", xor_mask);
            word = word.bitxor(xor_mask);
            // println!("after : {:#018b}", word);
            // println!("after no pad: {:#b}", word);
        } else {
            k -= 1
        }
    }
}


fn main() {
    let crc_gen: u8 = 0x1D;
    let default_string = "abcdefghijklmnopqrstuvwxyz";
    let string = std::env::args().nth(1).unwrap_or(default_string.to_string());
    let chunks_n = (string.len() + 4) / 5;
    let mut rng = rand::thread_rng();
    for chunk_n in 0..chunks_n {
        let mut chunk_as_number = 0u64;
        for i in 0..5 {
            let idx = chunk_n * 5 + i;
            if idx < string.len() {
                chunk_as_number.bitxor_assign((string.as_bytes()[idx] as u64).shl(8 * i as u64));
            }
        }
        let crc = div_xor(chunk_as_number << 8, crc_gen);
        println!("chunk: {}", &string[chunk_n * 5..min((chunk_n + 1) * 5, string.len())]);
        println!("crc: {}", crc);
        let string_corrected = {
            let mut str = string[chunk_n * 5..min((chunk_n + 1) * 5, string.len())].to_string();
            str.push(crc.into());
            str
        };
        println!("encoded string: {}", string_corrected);
        if chunk_n % 3 == 0 {
            println!("Trying add error");
            let bit_changes = rng.gen_range(1..3) * 2 + 1; // odd number less than 8
            for _ in 0..bit_changes {
                let rnd_bit_mask = 1_u64.shl(rng.gen_range(0..(5 * 8)));
                chunk_as_number.bitxor_assign(rnd_bit_mask);
            }
        }
        let error_detected = div_xor(chunk_as_number.shl(8u64).bitxor(crc as u64), crc_gen) != 0u8;
        println!("Error detected: {}", error_detected);
        println!()
    }
}
