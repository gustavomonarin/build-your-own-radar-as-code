#[macro_use]
extern crate lazy_static;
extern crate regex;

mod blip_document;

use structopt::StructOpt;
use serde::{Serialize, Deserialize};
use std::{fs, io, process};
use std::path::{Path, PathBuf};
use std::error::Error;
use std::borrow::Borrow;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Blip {
    name: String,
    quadrant: String,
    ring: String,
    is_new: bool,
    description: String,
}

fn convert_from_blips_yaml_files_to_csv_file(input_folder: &Path, output_file: &Path) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path(output_file).unwrap();

    let paths = fs::read_dir(input_folder).unwrap();
    paths
        .filter(is_valid_blip_file)
        .flat_map(|p|
            p.map(|s|
                fs::File::open(s.path())))
        .flat_map(|f|
            f.map(deserialize_blip_yaml)
        )
        .for_each(|b|
            match b {
                Ok(blip) => writer.serialize(blip).expect("Failed to write "),
                Err(err) => eprintln!("Error processing blip {}", err)
            }
        );

    writer.flush().unwrap();

    Ok(())
}

fn deserialize_blip_yaml(reader: fs::File) -> serde_yaml::Result<Blip> {
    serde_yaml::from_reader(reader)
}

fn is_valid_blip_file(entry: &Result<fs::DirEntry, io::Error>) -> bool {
    entry.as_ref().map_or(
        false,
        |e| e.file_name().to_str().unwrap().ends_with(".yaml"),
    )
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
        println!("{}", err);
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