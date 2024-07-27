use bytes::Bytes;
use camino::Utf8Path;
use std::fs;
use std::{error::Error, fs::File, io::Write};
use zip_extensions::zip_create_from_directory;

pub const SPINNER_LONG: &'static [&'static str] = &[
    "▐               ▌",
    "▐=              ▌",
    "▐==             ▌",
    "▐===            ▌",
    "▐ ===           ▌",
    "▐  ===          ▌",
    "▐   ===         ▌",
    "▐    ===        ▌",
    "▐     ===       ▌",
    "▐      ===      ▌",
    "▐       ===     ▌",
    "▐        ===    ▌",
    "▐         ===   ▌",
    "▐          ===  ▌",
    "▐           === ▌",
    "▐            ===▌",
    "▐             ==▌",
    "▐              =▌",
];

pub const SPINNER_SHORT: &'static [&'static str] = &[
    "[    ]", "[=   ]", "[==  ]", "[=== ]", "[====]", "[ ===]", "[  ==]", "[   =]", "[    ]",
    "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]",
];

pub fn is_int(value: f64) -> bool {
    (value.fract() < 1e-6) || ((1.0 - value.fract()) < 1e-6)
}

pub fn get_fractional(num: f64) -> u32 {
    let x = num
        .to_string()
        .split(".")
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Split from middle");

    x.get(1).expect("to second thing be there").to_owned()
}

pub fn create_folder(path: &Utf8Path) -> Result<(), Box<dyn Error>> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn get_chapter_string(chapter: f64) -> String {
    let is_point_chapter = !is_int(chapter);

    if is_point_chapter {
        format!("{:0>4}.{}", chapter.trunc(), get_fractional(chapter))
    } else {
        format!("{:0>4}", chapter)
    }
}

pub fn write_file(path: &Utf8Path, content: Bytes) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    file.write_all(&content)?;
    Ok(())
}

pub fn zip_rename_delete(chapter_path: &Utf8Path, chapter_folder_name: &str) {
    let zip_path = &chapter_path
        .parent()
        .unwrap()
        .join(format!("{}.zip", &chapter_folder_name));
    let cbz_path = &chapter_path
        .parent()
        .unwrap()
        .join(format!("{}.cbz", &chapter_folder_name));

    let _ = match zip_create_from_directory(
        &zip_path.as_std_path().to_path_buf(),
        &chapter_path.as_std_path().to_path_buf(),
    ) {
        Ok(()) => {}
        Err(err) => panic!("[ERROR] Couldn't create zip {:?}", err),
    };

    let _ = match fs::rename(zip_path, cbz_path) {
        Ok(()) => {}
        Err(err) => panic!("[ERROR] Couldn't rename zip {:?}", err),
    };

    let _ = match fs::remove_dir_all(chapter_path) {
        Ok(()) => {}
        Err(err) => panic!("[ERROR] Couldn't Delete Folder {:?}", err),
    };
}
