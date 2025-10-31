use byten::{Decode, DecodeOwned, var};
use byten::{Encode, Measure, prim::U16BE};
use byten::prelude::EncodeToVec;

#[derive(Debug, Encode, Measure, DecodeOwned)]
pub struct IcmpHeader {
    pub icmp_type: u8,
    pub code: u8,
    #[byten(U16BE)]
    pub checksum: u16,
    pub rest_of_header: [u8; 4],
}

#[derive(Debug, Encode, Measure, DecodeOwned)]
pub struct IcmpPacket {
    pub header: IcmpHeader,
    #[byten(var::Remaining)]
    pub data: Vec<u8>,
}

fn main() {
    let header = IcmpHeader {
        icmp_type: 8,
        code: 0,
        checksum: 0x1234,
        rest_of_header: [0, 2, 4, 8],
    };

    let packet = IcmpPacket {
        header,
        data: vec![1, 2, 3, 4, 5],
    };

    let encoded = packet.encode_to_vec().unwrap();
    println!("Encoded ICMP Packet: {:?}", encoded);

    let decoded = IcmpPacket::decode(&encoded, &mut 0).unwrap();
    println!("Decoded ICMP Packet: {:?}", decoded);
}
