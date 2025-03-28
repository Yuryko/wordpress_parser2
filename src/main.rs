use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use zip::ZipArchive;
use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_1252;
use regex::Regex;
use std::fs;
use std::io::BufRead;
use walkdir::WalkDir;
use zip;


// не знаю по каким причинам в encoding нет СP437, ок, сделаем вручную

pub fn cp437_to_utf8(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| {
        if b < 0x80 {
            b as char
        } else {
            match b {
                0x80 => 'Ç', 0x81 => 'ü', 0x82 => 'é', 0x83 => 'â', 0x84 => 'ä',
                0x85 => 'à', 0x86 => 'å', 0x87 => 'ç', 0x88 => 'ê', 0x89 => 'ë',
                0x8A => 'è', 0x8B => 'ï', 0x8C => 'î', 0x8D => 'ì', 0x8E => 'Ä',
                0x8F => 'Å', 0x90 => 'É', 0x91 => 'æ', 0x92 => 'Æ', 0x93 => 'ô',
                0x94 => 'ö', 0x95 => 'ò', 0x96 => 'û', 0x97 => 'ù', 0x98 => 'ÿ',
                0x99 => 'Ö', 0x9A => 'Ü', 0x9B => '¢', 0x9C => '£', 0x9D => '¥',
                0x9E => '₧', 0x9F => 'ƒ', 0xA0 => 'á', 0xA1 => 'í', 0xA2 => 'ó',
                0xA3 => 'ú', 0xA4 => 'ñ', 0xA5 => 'Ñ', 0xA6 => 'ª', 0xA7 => 'º',
                0xA8 => '¿', 0xA9 => '⌐', 0xAA => '¬', 0xAB => '½', 0xAC => '¼',
                0xAD => '¡', 0xAE => '«', 0xAF => '»', 0xB0 => '░', 0xB1 => '▒',
                0xB2 => '▓', 0xB3 => '│', 0xB4 => '┤', 0xB5 => '╡', 0xB6 => '╢',
                0xB7 => '╖', 0xB8 => '╕', 0xB9 => '╣', 0xBA => '║', 0xBB => '╗',
                0xBC => '╝', 0xBD => '╜', 0xBE => '╛', 0xBF => '┐', 0xC0 => '└',
                0xC1 => '┴', 0xC2 => '┬', 0xC3 => '├', 0xC4 => '─', 0xC5 => '┼',
                0xC6 => '╞', 0xC7 => '╟', 0xC8 => '╚', 0xC9 => '╔', 0xCA => '╩',
                0xCB => '╦', 0xCC => '╠', 0xCD => '═', 0xCE => '╬', 0xCF => '╧',
                0xD0 => '╨', 0xD1 => '╤', 0xD2 => '╥', 0xD3 => '╙', 0xD4 => '╘',
                0xD5 => '╒', 0xD6 => '╓', 0xD7 => '╫', 0xD8 => '╪', 0xD9 => '┘',
                0xDA => '┌', 0xDB => '█', 0xDC => '▄', 0xDD => '▌', 0xDE => '▐',
                0xDF => '▀', 0xE0 => 'α', 0xE1 => 'ß', 0xE2 => 'Γ', 0xE3 => 'π',
                0xE4 => 'Σ', 0xE5 => 'σ', 0xE6 => 'µ', 0xE7 => 'τ', 0xE8 => 'Φ',
                0xE9 => 'Θ', 0xEA => 'Ω', 0xEB => 'δ', 0xEC => '∞', 0xED => 'φ',
                0xEE => 'ε', 0xEF => '∩', 0xF0 => '≡', 0xF1 => '±', 0xF2 => '≥',
                0xF3 => '≤', 0xF4 => '⌠', 0xF5 => '⌡', 0xF6 => '÷', 0xF7 => '≈',
                0xF8 => '°', 0xF9 => '∙', 0xFA => '·', 0xFB => '√', 0xFC => 'ⁿ',
                0xFD => '²', 0xFE => '■', 0xFF => ' ',
                _ => '?', // Неизвестные символы заменяем на '?'
            }
        }
    }).collect()
}

pub fn unzip() -> io::Result<()> {

    // Открываем файл hugo-export.zip
    let file = File::open("hugo-export.zip")?;
    let mut archive = ZipArchive::new(file)?;

    // Проходим по всем файлам в архиве
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let raw_name = file.name_raw().to_vec(); // Получаем сырое имя файла в CP437  
        let decoded_name = cp437_to_utf8(&raw_name);// Декодируем в UTF-8
        let outpath = Path::new(&decoded_name);

        // Создаем директории, если они не существуют
        if let Some(p) = outpath.parent() {
            if !p.exists() {
                std::fs::create_dir_all(p)?;
            }
        }

        // Если это файл, записываем его на диск
        if file.is_file() {
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    println!("Файл успешно разархивирован с перекодировкой названий!");
    Ok(())
}



// удалим все лишние посты

fn del_post() -> io::Result<()> {
    let dir_path = "."; // путь к каталогу, который нужно обработать

    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            if !contains_string(file_path, "Для внешней публикации")? {
                println!("Удаление файла: {:?}", file_path);
                fs::remove_file(file_path)?;
            }
        }
    }

    Ok(())
}

fn contains_string(file_path: &std::path::Path, search_string: &str) -> io::Result<bool> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains(search_string) {
            return Ok(true);
        }
    }

    Ok(false)
}

// надо переделать все регулярки из питона
//pub fn delete() io::Result<()> {
//    let regex = Regex::new(r"\d{3}").unwrap();
//    let a: &str = "123bbasfsdf23asd2021-06-17";
    //files = os.listdir(root_dir)
    //for each_file in files:
    //    full_path = "%s/%s" % (root_dir, each_file)
    //   each_file_content = open(full_path, 'r', encoding="utf-8").read()
    //    if not any(word in each_file_content for word in words):
    //       os.unlink(full_path)

//}

fn main(){
    unzip();
    del_post();


}