use serde::{Serialize, Deserialize};
use std::{fs, io};

#[derive(Deserialize, Serialize, Debug, )]
#[serde(rename_all="camelCase")]
struct Blip {
    name: String,
    quadrant: String,
    ring: String,
    is_new: bool,
    description: String,
}

fn main() {
    let mut writer = csv::Writer::from_writer(io::stdout());

    let paths = fs::read_dir("./blips-thoughtworks-vol21/").unwrap();

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