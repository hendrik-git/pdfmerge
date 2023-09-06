use clap::{App, Arg};
//use std::env;
use std::error::Error;
//use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    out_file: Option<String>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("pdfmerge")
        .version("0.1.0")
        .author("Hendrik Pöttker")
        .about("merge pdfs")
        .arg(
            Arg::with_name("out_file")
                .value_name("OUT")
                .help("Output file name")
                .multiple(false)
                .default_value("merged.pdf"),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .get_matches();
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        out_file: matches.value_of("out_file").map(String::from),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("Hello world from the run function:\n{:?}", config);

    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(_file) => {
                // for line_result in file.lines() {
                //     let line = line_result?;
                //     println!("{}", line);
                // }
            }
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
