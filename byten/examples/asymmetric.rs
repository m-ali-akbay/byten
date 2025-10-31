use byten::{Decode, Encode, Measure, SelfCodec, prelude::EncodeToVec as _, prim, util::{Asymmetric, ConvertDecoded, ConvertEncoded}, var};

type U8AsUSize = Asymmetric<
    ConvertEncoded<SelfCodec<u8>, usize>,
    ConvertDecoded<usize, SelfCodec<u8>>,
>;
type U16BEAsUSize = Asymmetric<
    ConvertEncoded<prim::U16BE, usize>,
    ConvertDecoded<usize, prim::U16BE>,
>;

#[derive(Debug, Encode, Measure, Decode)]
pub struct Directory {
    #[byten(var::String::<U8AsUSize>::default())]
    pub name: String,
    #[byten(var::Vec::<U16BEAsUSize, SelfCodec<Entry>>::default())]
    pub entries: Vec<Entry>,
}

#[derive(Debug, Encode, Measure, Decode)]
pub struct File {
    #[byten(var::String::<U8AsUSize>::default())]
    pub name: String,
    #[byten(var::Vec::<U8AsUSize, SelfCodec<_>>::default())]
    pub content: Vec<u8>,
}

#[derive(Debug, Encode, Measure, Decode)]
#[repr(u8)]
pub enum Entry {
    File(File) = 1,
    Directory(Directory) = 2,
}

fn main() {
    let dir = Directory {
        name: "root".to_string(),
        entries: vec![
            Entry::File(File {
                name: "file1.txt".to_string(),
                content: b"Hello, World!".to_vec(),
            }),
            Entry::Directory(Directory {
                name: "subdir".to_string(),
                entries: vec![
                    Entry::File(File {
                        name: "file2.txt".to_string(),
                        content: b"Rust is awesome!".to_vec(),
                    }),
                ],
            }),
        ],
    };

    let encoded = dir.encode_to_vec().unwrap();
    println!("Encoded Directory: {:?}", encoded);

    let mut offset = 0;
    let decoded_dir = Directory::decode(&encoded, &mut offset).unwrap();
    println!("Decoded Directory: {:?}", decoded_dir);
}
