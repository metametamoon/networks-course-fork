extern crate core;

use std::ops::BitXorAssign;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn generate_checksum(data: &[u8], k: usize) -> Vec<u8> {
    let mut res = Vec::<u8>::new();
    for i in 0..k {
        let mut it = i as usize;
        let mut total: u8 = 0;
        while it < data.len() {
            total.bitxor_assign(data[it]);
            it += k as usize;
        }
        res.push(total)
    }
    res
}

fn extract_data(checksummed_data: &[u8], k: usize) -> Option<Vec<u8>> {
    let checksum = &checksummed_data[0..k];
    let payload = &checksummed_data[k..];
    if generate_checksum(payload, k) != checksum {
        None
    } else {
        Some(payload.to_vec())
    }
}

pub fn create_packet(bit: u8, data: &[u8]) -> Vec<u8> {
    let mut result = Vec::<u8>::new();
    result.push(bit);
    for d in data {
        result.push(*d)
    }
    result
}

pub struct Packet {
    pub bit: u8,
    pub data: Vec<u8>,
}

pub fn unwrap_packet(raw_packet: &[u8]) -> Option<Packet> {
    let bit = raw_packet[0];
    let rest = &raw_packet[1..];
    Packet {
        bit,
        data: rest.to_vec()
    }.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extraction_also_works() {
        let k = 16;
        let payload = [0u8, 1u8, 1u8, 0u8, 1u8, 0u8];
        let checksum = generate_checksum(&payload, k);
        let mut checksummed_data = Vec::<u8>::new();
        checksummed_data.append(&mut checksum.to_vec());
        checksummed_data.append(&mut payload.to_vec());
        {
            let extracted_payload = extract_data(&checksummed_data, k).unwrap();
            assert_eq!(extracted_payload, payload);
        } 
        {
            let mut changed_data = checksummed_data.clone();
            changed_data[1] = 1 - changed_data[1];
            let extracted_payload = extract_data(&changed_data, k);
            assert!(extracted_payload.is_none());
        }
    }
    
    #[test]
    fn short_tests() {
        let data = [0u8, 1u8, 1u8, 1u8];
        assert_eq!(generate_checksum(&data, 2), [1u8, 0u8].to_vec());
        assert_eq!(generate_checksum(&data, 3), [1u8, 1u8, 1u8].to_vec());
        assert_eq!(
            generate_checksum(&data, 5),
            [0u8, 1u8, 1u8, 1u8, 0u8].to_vec()
        );
    }
}
