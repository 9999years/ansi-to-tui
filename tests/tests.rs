use ansi_to_tui::ansi_to_text;

#[test]
#[ignore]
fn test_bytes() {
    let bytes = vec![27_u8, 91, 51, 49, 109, 65, 65, 65];
    println!("{:#?}", ansi_to_text(&bytes))
}

#[test]
fn text_unicode() {
    let bytes = "AAA🅱️🅱️🅱️".as_bytes();
    println!("{:?}", ansi_to_text(bytes));
}

#[test]
fn ascii_rgb() {
    use crate::ansi_to_text;
    let bytes = b"\x1b[38;2;100;100;100mAAABBB";
    println!("{:#?}", ansi_to_text(&bytes));
}

#[test]
#[ignore = "Give a lot of output"]
fn archlinux_ascii() {
    use crate::ansi_to_text;
    use std::{fs::File, io::Read};
    let mut ascii = File::open("tests/archlinux.ascii").unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    ascii.read_to_end(&mut buffer).unwrap();
    let text = ansi_to_text(&buffer).unwrap();
    for line in text.lines {
        println!("{:?}", line.width());
    }
}

#[test]
#[ignore]
fn ascii_test() {
    use crate::ansi_to_text;
    use std::{fs::File, io::Read};
    let mut ascii = File::open("tests/ascii").unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    ascii.read_to_end(&mut buffer).unwrap();
    let text = ansi_to_text(&buffer).unwrap();
    println!("{:#?}", text);
}
