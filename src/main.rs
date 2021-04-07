#[macro_use]
extern crate lazy_static;
extern crate regex;

mod blip_document;

use structopt::StructOpt;
use std::{fs, process};
use std::path::{Path, PathBuf};
use std::error::Error;
use std::borrow::Borrow;
use glob::glob;
use crate::blip_document::BlipDocument;
use std::ops::Add;

/// this should go to a "domain" area and leave the main only with command line handling
fn convert_from_blips_yaml_files_to_csv_file(input_folder: &Path, output_file: &Path) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path(output_file)?;

    let input_folder_string = input_folder.to_str()
        .ok_or("")?.to_string();

    let blips_files_pattern = input_folder_string.add("/**/*.md");

    println!("Scanning for blip files using the pattern  {}", blips_files_pattern);

    for entry in glob(blips_files_pattern.as_str())? {
        let blip_file = fs::File::open(entry?)?;

        let blip_document = BlipDocument::parse(blip_file)?;

        writer.serialize(blip_document)?;
    }

    writer.flush().unwrap();

    Ok(())
}

#[derive(StructOpt, Debug)]
#[structopt(name = "blipnize",
about = "A command to convert simple yaml blip files to the format expected by the thoughtworks radar")]
struct CommandArgs {
    /// The input dir containing the multiple blip yaml files. The files must have the .md extension
    #[structopt(short = "d", long = "--input-dir")]
    input_dir: PathBuf,

    /// The output file containing the aggregated blips in the expected csv format
    #[structopt(short = "o", long = "--output-file")]
    output_file: PathBuf,
}

fn main() {
    let args: CommandArgs = CommandArgs::from_args();
    validate_input_dir(args.input_dir.borrow());
    validate_output_file(args.output_file.borrow());

    if let Err(err) = convert_from_blips_yaml_files_to_csv_file(
        args.input_dir.borrow(),
        args.output_file.borrow(),
    ) {
        eprintln!("{}", err);
        process::exit(1);
    }
}

fn validate_input_dir(input_dir: &Path) {
    if !input_dir.exists() {
        eprintln!("error: Input dir does not exists '{}' .", input_dir.display());
        process::exit(1); // should avoid multiple exits. TODO: convert everything to a Result and chain them
    }
}

fn validate_output_file(output_file: &Path) {
    if output_file.exists() {
        eprintln!("error: Output file '{}' already exists.", output_file.display());
        process::exit(1); // should avoid multiple exits. TODO: convert everything to a Result and chain them
    }
}