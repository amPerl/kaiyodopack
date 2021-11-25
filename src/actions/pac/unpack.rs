use anyhow::Context;
use binrw::{BinRead, BinReaderExt, NullString};
use clap::Parser;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

#[derive(Parser, Debug)]
pub struct UnpackOpts {
    #[clap(short = 'i', long, about = "input file")]
    input_path: String,
    #[clap(short = 'o', long, about = "output directory")]
    output_dir: String,
}

pub fn unpack_pac(opts: UnpackOpts) -> anyhow::Result<()> {
    let mut file = File::open(&opts.input_path)?;

    let pac = file.read_le::<Pac>()?;

    let compressed = (pac.flags & 1) > 0;
    let ciphered = (pac.flags & 2) > 0;

    // dbg!(&compressed, &ciphered);

    for entry in pac.entries {
        // dbg!(&entry.name);

        let compressed_size = entry.length as usize;

        file.seek(SeekFrom::Start(entry.position as u64))?;
        let mut data = vec![0; compressed_size];
        file.read_exact(&mut data)?;

        if ciphered {
            for b in data.iter_mut() {
                *b ^= 0xFE;
            }
        }

        if compressed {
            let mut decompressed = vec![0u8; entry.decompressed_length as usize];
            super::compression::decompress(&data, &mut decompressed);
            data = decompressed;
        }

        let output_file_path = Path::new(&opts.output_dir).join(&entry.name.into_string());
        let output_file_dir = output_file_path
            .parent()
            .context("could not get output file path parent dir")?;

        std::fs::create_dir_all(output_file_dir)?;

        let mut output_file = File::create(output_file_path)?;
        output_file.write_all(&data)?;
    }

    Ok(())
}

#[derive(BinRead, Debug)]
#[br(magic = b"kzpack2^")]
struct Pac {
    entry_count: u32,
    flags: u32,
    #[br(count = entry_count)]
    entries: Vec<Entry>,
}

#[derive(BinRead, Debug)]
struct Entry {
    #[br(pad_size_to = 512)]
    name: NullString,
    position: u32,
    decompressed_length: u32,
    length: u32,
    timestamp: u32,
    unk_name: [u8; 4],
    unk5: u32,
}
