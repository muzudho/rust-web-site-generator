extern crate rust_web_site_generator;

use rust_web_site_generator::*;

/// # 実行方法
/// [Windows]+[R], "cmd",
/// 
/// ```
/// ### コンパイル。
/// cd C:\MuzudhoDrive\projects_rust\rust-web-site-generator
/// cargo clippy --example main
/// 
/// ### 実行。
/// cls
/// cargo run --example main
/// ```

fn main() {
    // ディレクトリを指定しろだぜ☆（＾～＾）
    let home_dir = "C:\\muzudho-auto-generated-web";

    // ディレクトリの存在を確認しろだぜ☆（＾～＾）
    if exists_path(home_dir) {
        println!("Hello, {}!", home_dir);

        // index.html の存在を確認しろだぜ☆（＾～＾）
        let index_html = &format!("{}/index.html",home_dir);
        if exists_path(index_html){
            println!("Hello, {}!", index_html);

        } else {
            // index.html が無ければ、作れだぜ☆（＾～＾）
            println!("Create: '{}'.", index_html); 

            // ページを新規作成する。
            create_new_page(index_html);
        }

        // ランダムなディレクトリ名を考えたいぜ☆（＾～＾）
        let dir_name = &create_dir_name();
        let dir_path = &format!("{}/{}", home_dir, dir_name);
        if !exists_path(dir_path){
            // ディレクトリが無ければ、作れだぜ☆（＾～＾）
            if !create_dir(dir_path) {
                println!("Create '{}' directory fail. bye.", dir_path); 
                return;
            }
        }

        // ランダムなファイル名を考えたいぜ☆（＾～＾）
        let page_name = &create_page_name();

        // ページを作ろう☆（＾～＾）
        let new_page_html = &format!("{}/{}", dir_path, page_name);
        if !exists_path(new_page_html){
            // そのページが無ければ、作れだぜ☆（＾～＾）
            create_new_page(new_page_html);
        }        

        // ホームディレクトリの中から１つのディレクトリを選ぶぜ☆（＾～＾）
        match choose_dir(home_dir) {
            Some(chose_dir) => {
                println!("Chose: {}.", chose_dir);

                // ディレクトリの中から１つのファイルを選ぶぜ☆（＾～＾）
                match choose_file(&chose_dir) {
                    Some(chose_file) => {
                        println!("Chose: {}.", chose_file);

                        // そのページに何かを追加☆（＾～＾）
                        let relative_path = format!("../{}/{}", dir_name, page_name);
                        let link = format!("<a href=\"{}\">{}</a>\n", relative_path, relative_path);
                        append_text_to_page(&chose_file, &link);
                    },
                    None => {
                        println!("Not found child file.");
                    }
                }
            },
            None => {
                println!("Not found child directory.");
            }
        }

    } else {
        // 指定のディレクトリがないときは、おとなしく何もしないぜ☆（＾～＾）
        println!("Not found '{}'. bye.", home_dir); 
    }
}
