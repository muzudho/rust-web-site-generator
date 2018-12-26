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

    } else {
        // 指定のディレクトリがないときは、おとなしく何もしないぜ☆（＾～＾）
        println!("Not found '{}'. bye.", home_dir); 
    }
}
