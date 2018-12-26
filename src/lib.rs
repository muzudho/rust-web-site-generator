/// バージョン探すなら https://crates.io/
/// ファイル操作なら https://qiita.com/fujitayy/items/12a80560a356607da637
extern crate rand;
use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::str::Chars;
use std::fs::{self, File, metadata};
use std::io::{self, BufReader, BufWriter, Read, Write};
use rand::prelude::IteratorRandom;

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

/// 指定ディレクトリから、ディレクトリを１つ選ぶ。
pub fn choose_dir(search_path:&str) -> Option<String> {
    let paths = fs::read_dir(search_path).unwrap();

    // ディレクトリを詰め込む☆（＾～＾）
    let mut vec = Vec::new();

    for path in paths {
        let path_str = path.unwrap().path();
        let meta = metadata(&path_str).unwrap();

        if meta.is_dir() {
            println!("Dir: {}", path_str.display());
            vec.push(path_str);
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

/// 指定ディレクトリから、ファイルを１つ選ぶ。
pub fn choose_file(search_path:&str) -> Option<String> {
    let paths = fs::read_dir(search_path).unwrap();

    // ディレクトリを詰め込む☆（＾～＾）
    let mut vec = Vec::new();

    for path in paths {
        let path_str = path.unwrap().path();
        let meta = metadata(&path_str).unwrap();

        if meta.is_file() {
            println!("File: {}", path_str.display());
            vec.push(path_str);
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

/// ランダムでひらがなをn文字返す。空白、句読点も含む。
/// https://qiita.com/Linda_pp/items/d363ceff3c37bc9a4ae6
pub fn create_hiragana(count:usize) -> String {
    let v: Vec<char> = "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわをんがぎぐげござじずぜぞだぢづでどばびぶべぼぱぴぷぺぽぁぃぅぇぉっゃゅょ、。　".chars().collect();
    let mut rng = rand::thread_rng();
    let name: String = (0..count).map(|_| v.choose(&mut rng).unwrap()).collect();
    name
}

/// ページに文字を書き込む。
pub fn append_text_to_page(path:&str, text:&str) {
    let mut content;

    // ファイルを開く。
    {
        content = fs::read_to_string(path).unwrap();

        // </body> を探す。
        match content.find("</body>") {
            Some(index) => {
                println!("Body: {}.", index);
                content.insert_str(index, text);
            },
            None => {
                // 無ければ何もしない。
                println!("'</body>' は無い☆（＾～＾）");
                return;
            }
        }
    }

    // ファイルを上書きする。
    {
        let mut f = BufWriter::new(fs::File::create(path).unwrap());
        f.write_all(content.to_string().as_bytes()).unwrap();
        println!("Write: {}.", path);
    }
}