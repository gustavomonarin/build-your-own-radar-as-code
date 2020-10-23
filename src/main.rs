use serde::{Serialize, Deserialize};
use std::{fs, io, process};
use std::path::Path;
use std::error::Error;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all="camelCase")]
struct Blip {
    name: String,
    quadrant: String,
    ring: String,
    is_new: bool,
    description: String,
}

fn convert_from_blips_yaml_files_to_csv_file(input_folder: &Path, output_file: &Path)  -> Result<(), Box<dyn Error>>  {

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


fn main() {

    let path = Path::new("./blips-thoughtworks-vol21/");

    let output = Path::new("/tmp/output.csv");
    validate_output_file(output);

    if let Err(err) = convert_from_blips_yaml_files_to_csv_file(path, output) {
        println!("{}", err);
        process::exit(1);
    }

}

fn validate_output_file(output_file: &Path) {
    if output_file.exists() {
        eprintln!("ERROR: Output file '{}' already exists.", output_file.display());
        process::exit(1); // should avoid multiple exits. TODO: convert everything to a Result and chain them
    }
}