use std::fs;
use std::io::{BufWriter, Write};

pub fn exists_path(path:&str) -> bool { 
    fs::metadata(path).is_ok()
} 

/// HTMLページを新規作成する。
pub fn create_new_page(index_html:&str) {
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