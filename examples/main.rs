extern crate rust_web_site_generator;

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
    if path_exists(home_dir) {
        println!("Hello, {}!", home_dir);

    } else {
        // 指定のディレクトリがないときは、おとなしく何もしないぜ☆（＾～＾）
        println!("Not found '{}'. bye.", home_dir); 
    }
}
