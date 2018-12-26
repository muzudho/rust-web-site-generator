/// バージョン探すなら https://crates.io/
extern crate rand;
use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::fs;
use std::fs::metadata;
use std::io::{BufWriter, Write};

pub fn exists_path(path:&str) -> bool { 
    fs::metadata(path).is_ok()
} 

/// ディレクトリを新規作成する。
pub fn create_dir(path:&str) -> bool {
    match fs::create_dir(path) {
        Err(e) => {
            println!("{}: {}", path, e);
            false
        },
        Ok(_) => {
            println!("Create: '{}'.", path);
            true
        },
    }
}

/// 指定ディレクトリにある、ディレクトリを探す。
pub fn find_dir(search_path:&str) -> Option<String> {
    let paths = fs::read_dir(search_path).unwrap();

    // ディレクトリを詰め込む☆（＾～＾）
    let mut vec = Vec::new();

    for path in paths {
        let path_str = path.unwrap().path();
        let meta = metadata(&path_str).unwrap();

        if meta.is_dir() {
            println!("Dir: {}", path_str.display());
            vec.push(path_str);

        } else if meta.is_file() {
            println!("File: {}", path_str.display());

        } else {
            println!("What: {}", path_str.display());

        }
    }

    // ランダムに１つ選ぶ。
    let mut rng = thread_rng();
    if (*vec).is_empty() {
        None
    } else {
        Some(vec.choose(&mut rng).unwrap().display().to_string())
    }
}

/// HTMLページを新規作成する。
pub fn create_new_page(index_html:&str) {
    println!("Create: '{}'...", index_html); 

    let html = "<!DOCTYPE html>
<html>
<head>
<title>Page Title</title>
</head>
<body>

<h1>This is a Heading</h1>
<p>This is a paragraph.</p>

</body>
</html>";

    let mut f = BufWriter::new(fs::File::create(index_html).unwrap());
    f.write_all(html.to_string().as_bytes()).unwrap();
}

/// ランダムな4文字のHTMLファイル名を生成する。
/// 26^4 は 456,976 だぜ☆（＾～＾）
/// 卑猥な名前や、ファイルに使えない名前が出てくるかもしれないが、パスで☆（＾～＾）
pub fn create_page_name() -> String {
    const CHARS: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz";
    let mut rng = rand::thread_rng();

    let name: String = (0..4).map(|_| *rng.choose(&CHARS).unwrap() as char).collect();
    format!("{}.html", name)
}

/// ランダムな2文字のディレクトリ名を生成する。
/// 26^2 は 676 だぜ☆（＾～＾） たまに被るぐらいがいい☆（＾～＾） ディレクトリが大量に作られても困るだろ☆（＾～＾）
/// ディレクトリに使えない名前もあるかもしれないが、パスで☆（＾～＾）
pub fn create_dir_name() -> String {
    const CHARS: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz";
    let mut rng = rand::thread_rng();

    let name: String = (0..2).map(|_| *rng.choose(&CHARS).unwrap() as char).collect();
    format!("{}", name)
}
