use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use zip::ZipArchive;
use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_437;
use regex::Regex;

pub fn unzip() -> io::Result<()> {
    // Открываем файл file.zip
    let file = File::open("file.zip")?;
    let mut archive = ZipArchive::new(file)?;

    // Проходим по всем файлам в архиве
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let raw_name = file.name_raw().to_vec(); // Получаем сырое имя файла в CP437
        let decoded_name = WINDOWS_437.decode(&raw_name, DecoderTrap::Strict).unwrap(); // Декодируем в UTF-8

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

pub fn delete() io::Result<()> {
    let regex = Regex::new(r"\d{3}").unwrap();
    let a: &str = "123bbasfsdf23asd2021-06-17";
    //files = os.listdir(root_dir)
    //for each_file in files:
    //    full_path = "%s/%s" % (root_dir, each_file)
    //   each_file_content = open(full_path, 'r', encoding="utf-8").read()
    //    if not any(word in each_file_content for word in words):
    //       os.unlink(full_path)

}

fn main(){
    use::unzip;
    unzip();


}