use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use zip::ZipArchive;
use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_1252;

fn main() -> io::Result<()> {
    let file = File::open("hugo-export.zip")?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let raw_name = file.name_raw().to_vec();
        let decoded_name = WINDOWS_1252.decode(&raw_name, DecoderTrap::Strict).unwrap(); // Декодируем в UTF-8

        let outpath = Path::new(&decoded_name);

        if let Some(p) = outpath.parent() {
            if !p.exists() {
                std::fs::create_dir_all(p)?;
            }
        }

        if file.is_file() {
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    println!("Файл успешно разархивирован!");
    Ok(())
}
