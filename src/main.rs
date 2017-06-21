use std::fs::{self, File};
use std::io::prelude::*;
use std::process::Command;

extern crate image;
use image::GenericImage;

fn main() {
    let filename = std::env::args().nth(1).expect("No filename provided!");
    let main_filename = filename.split('.').nth(0).expect("No before-extension?");
    let mut image = image::open(&filename).expect("Unable to open image!");
    let mut vstxt = File::create("vs.txt").expect("Unable to create vs.txt!");
    let (width, height) = image.dimensions();
    if width % 128 != 0 || height % 128 != 0 {
        panic!("Image's dimensions not a multiple of 128!");
    }
    let (num_h, num_v): (u32, u32) = (width / 128, height / 128);
    let mut part_filename = String::new();
    for v in 0..num_v {
        for h in 0..num_h {
            let current_num = v * num_h + h + 1;
            
            part_filename.clear();
            part_filename.push_str(&*main_filename);
            part_filename.push_str(&*(current_num.to_string()));
            part_filename.push_str(".bmp");
            
            let subimage = image.sub_image(h * 128, v * 128, 128, 128);
            let image = subimage.to_image();
            image.save(&part_filename).expect("Unable to save one of the parts!");
            write!(&mut vstxt, "{}{}", &part_filename, "\r\n").expect("Unable to write to vs.txt!");
        }
    }
    
    Command::new("COMPIMG.exe")
        .arg("-c255")
        .arg("-r0")
        .arg("-f0")
        .arg("-l0,0")
        .arg("-ow")
        .arg("-emov")
        .arg(String::from("-M") + &main_filename)
        .arg("+vs.txt")
        .output().expect("Unable to call COMPIMG.exe!");
    println!("Use as: {}{}h*{}v*.mov", main_filename, num_h, num_v);
    for v in 0..num_v {
        for h in 0..num_h {
            let tempfile = format!("{}{}.bmp", main_filename, v * num_h + h + 1);
            fs::remove_file(&tempfile).expect("Unable to delete a .bmp!");
        }
    }
    fs::remove_file("vs.txt").expect("Unable to delete vs.txt!");
}
