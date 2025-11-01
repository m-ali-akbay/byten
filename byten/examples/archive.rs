use std::ffi::CString;

use byten::{Decode, DecodeOwned, Encode, Measure, SelfCodec, prelude::EncodeToVec as _, prim, util::Convert, var};

type U16BEAsUSize = Convert<prim::U16BE, usize>;

#[derive(Debug, Encode, Measure, DecodeOwned)]
pub struct Directory {
    pub name: CString,
    #[byten(var::Vec::<U16BEAsUSize, SelfCodec<_>>::default())]
    pub entries: Vec<Box<Entry>>,
}

#[derive(Debug, Encode, Measure, DecodeOwned)]
pub struct File {
    pub name: CString,
    #[byten(var::Vec::<U16BEAsUSize, SelfCodec<_>>::default())]
    pub content: Vec<u8>,
}

#[derive(Debug, Encode, Measure, DecodeOwned)]
#[repr(u8)]
pub enum Entry {
    File(File) = 1,
    Directory(Directory) = 2,
}

fn main() {
    let dir = Directory {
        name: CString::new("root").unwrap(),
        entries: vec![
            Box::new(Entry::File(File {
                name: CString::new("file1.txt").unwrap(),
                content: b"Hello, World!".to_vec(),
            })),
            Box::new(Entry::Directory(Directory {
                name: CString::new("subdir").unwrap(),
                entries: vec![
                    Box::new(Entry::File(File {
                        name: CString::new("file2.txt").unwrap(),
                        content: b"Rust is awesome!".to_vec(),
                    })),
                ],
            })),
        ],
    };

    let encoded = dir.encode_to_vec().unwrap();
    println!("Encoded Directory: {:?}", encoded);

    let mut offset = 0;
    let decoded_dir = Directory::decode(&encoded, &mut offset).unwrap();
    println!("Decoded Directory: {:?}", decoded_dir);
}
