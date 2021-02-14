use std::collections::HashMap;
use std::io;
use std::error::Error;

use regex::Regex;
use std::str::FromStr;

/// Describes a blip document following the jekyll FrontMatter pattern.
///
/// see more:
///
/// https://github.com/jekyll/jekyll/blob/master/lib/jekyll/document.rb#L13
///
///
pub struct BlipDocument {
    name: String,
    quadrant: String,
    ring: String,
    is_new: bool,
    description: String,
}


impl BlipDocument {
    fn parse<R>(mut reader: R) -> Result<BlipDocument, Box<dyn Error>>
        where
            R: io::Read,
    {
        lazy_static! {
            static ref YAML_FRONT_MATTER_REGEXP: Regex =
                Regex::new(r"^(?s)\s*---(.*)---(.*)$").unwrap();
        }

        let mut document_full_content = String::new();
        reader.read_to_string(&mut document_full_content)?;

        let captures = YAML_FRONT_MATTER_REGEXP.captures(&mut document_full_content)
            .expect("Invalid document format. The blip document should follow the [front matter pattern]");

        let front_matter: HashMap<String, String> = captures.get(1)
            .map_or(
                HashMap::new(),
                |m| serde_yaml::from_str(m.as_str()).unwrap());

        let description = captures.get(2).map_or("", |m| m.as_str());

        Ok(
            BlipDocument {
                name: front_matter.get("name")
                    .expect("Missing mandatory field 'name'.")
                    .to_string(),
                quadrant: front_matter.get("quadrant")
                    .expect("Missing mandatory field 'quadrant'.")
                    .to_string(),
                ring: front_matter.get("ring")
                    .expect("Missing mandatory field 'ring'.")
                    .to_string(),
                is_new: FromStr::from_str(
                    front_matter.get("isNew")
                        .expect("Missing mandatory field 'name'."))
                    .unwrap(),
                description: description.to_string(),
            })
    }
}


#[cfg(test)]
mod tests {
    use crate::blip_document::BlipDocument;

    #[test]
    fn test_parse() -> Result<(), String> {
        let blip_file_content = "
---
name: Blip name
quadrant: Techniques
ring: Assess
isNew: true
---
The content here
".as_bytes();

        let document = BlipDocument::parse(blip_file_content).unwrap();

        assert_eq!(document.name, "Blip name".to_string());
        assert_eq!(document.quadrant, "Techniques".to_string());
        assert_eq!(document.ring, "Assess".to_string());
        assert_eq!(document.is_new, true);
        assert_eq!(document.description, "\nThe content here\n".to_string());
        Ok(())
    }


    #[test]
    #[should_panic(expected = "Invalid document format. The blip document should follow the [front matter pattern]")]
    fn test_parse_no_metadata() {
        let blip_file_content = "The content here".as_bytes();

        BlipDocument::parse(blip_file_content).unwrap();
    }

    #[test]
    #[should_panic(expected = "Missing mandatory field 'name'.")]
    fn test_parse_missing_name() {
        let blip_file_content = "
---
quadrant: Techniques
ring: Assess
isNew: true
---
The content here
".as_bytes();

        BlipDocument::parse(blip_file_content).unwrap();
    }

    #[test]
    fn test_parse_no_content() -> Result<(), String> {
        let blip_file_content = "
---
name: Blip name
quadrant: Techniques
ring: Assess
isNew: true
---".as_bytes();

        let document = BlipDocument::parse(blip_file_content).unwrap();

        assert_eq!(document.description, "".to_string());
        Ok(())
    }

}