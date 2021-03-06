extern crate clap;
extern crate colored;
extern crate dialoguer;
extern crate tar;

use clap::{App, Arg};
use colored::*;
use dialoguer::Confirmation;
use libflate::gzip::Decoder;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Read};
use std::path::Path;
use tar::Archive;

fn main() -> io::Result<()> {
    let matches = App::new("untar")
        .version("0.1.0")
        .author("Alexander Kluth <deralex@cpan.org>")
        .about("Unpack tar archives with various compression algorithms (gz, bzip2 etc.)")
        .arg(
            Arg::with_name("FILE")
                .help("tar archive to unpack")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = File::open(matches.value_of("FILE").unwrap()).unwrap();

    let decoder = Decoder::new(file).unwrap();
    let mut a = Archive::new(decoder);

    for file in a.entries().unwrap() {
        let mut file = file.unwrap();

        println!(
            "{} {:?}",
            "Extracting".green().bold(),
            file.header().path().unwrap()
        );

        let path = file.path()?;

        if path.to_str().unwrap().chars().last().unwrap() == '/' {
            if Path::new(path.to_str().unwrap()).exists() {
                continue;
            } else {
                fs::create_dir(path.to_str().unwrap())?;
                continue;
            }
        }

        let mut file_to_write;
        let mut data;

        if Path::new(path.to_str().unwrap()).exists() {
            if Confirmation::new()
                .with_text("File exists. Overwrite?")
                .interact()
                .unwrap()
            {
                file_to_write = File::create(file.header().path().unwrap())?;
                data = Vec::new();

                file.read_to_end(&mut data).unwrap();
                file_to_write.write_all(&data)?;
            }
        } else {
            file_to_write = File::create(file.header().path().unwrap())?;
            data = Vec::new();

            file.read_to_end(&mut data).unwrap();
            file_to_write.write_all(&data)?;
        }
    }

    Ok(())
}
