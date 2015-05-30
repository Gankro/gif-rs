extern crate gif;

use std::fs::{self, File};
use std::io::Result as IoResult;
use std::io::{Read, Write};
use std::env;

static out_dir: &'static str = "output";

fn main() {
    let mut args = env::args();
    args.next();

    let path = args.next().unwrap();

	if let Err(err) = do_it(&path) {
        println!("OH NO! {}", err);
    }
}

fn do_it(path: &str) -> IoResult<()> {
	let mut file = try!(File::open(path));

	let mut buf = vec![];
	try!(file.read_to_end(&mut buf));

	let gif = gif::parse_gif(&buf).unwrap();


    if fs::read_dir(out_dir).is_ok() {
        try!(fs::remove_dir_all(out_dir));
    }
    try!(fs::create_dir(out_dir));

    for (idx, frame) in gif.frames.iter().enumerate() {
        let image = RBitmap {
            width: frame.width,
            height: frame.height,
            data: &frame.data,
        };
        try!(save(&image, idx));
    }


	Ok(())
}

struct RBitmap<'a> {
	width: u16,
	height: u16,
	data: &'a [u8],
}

fn save<'a>(data: &RBitmap<'a>, num: usize) -> IoResult<()> {
    let file_name = format!("{}/frame-{:03}.tga", out_dir, num);
	let mut file = try!(File::create(&file_name));

	let mut header = [0; 18];
    header[2] = 2; // truecolor
    header[12] = data.width as u8 & 0xFF;
    header[13] = (data.width >> 8) as u8 & 0xFF;
    header[14] = data.height as u8 & 0xFF;
    header[15] = (data.height >> 8) as u8 & 0xFF;
    header[16] = 32; // bits per pixel

    try!(file.write_all(&header));

    // The image data is stored bottom-to-top, left-to-right
    for y in (0..data.height).rev() {
        for x in 0..data.width {
            let idx = (x as usize + y as usize * data.width as usize) * 4;
            let r = data.data[idx + 0];
            let g = data.data[idx + 1];
            let b = data.data[idx + 2];
            let a = data.data[idx + 3];
            try!(file.write_all(&[b, g, r, a]));
        }
    }


    // The file footer
    let footer = b"\0\0\0\0\0\0\0\0TRUEVISION-XFILE.\0";

    try!(file.write_all(footer));

    Ok(())
}