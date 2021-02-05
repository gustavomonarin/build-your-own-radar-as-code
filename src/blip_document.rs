use std::collections::HashMap;
use std::io;
use std::error::Error;

use regex::Regex;

/// Describes a blip document following the jekyll FrontMatter pattern.
///
/// see more:
///
/// https://github.com/jekyll/jekyll/blob/master/lib/jekyll/document.rb#L13
///
///
pub struct BlipDocument<'t> {
    /// The metadata for
    metadata: HashMap<String, String>,

    /// The textual content of the file, providing the detailed description of the blip.
    /// The format must be markdown.
    content: &'t str,
}


impl<'t> BlipDocument<'t> {
    fn parse<R>(mut reader: R) -> Result<BlipDocument<'t>, Box<dyn Error>>
        where
            R: io::Read,
    {
        // lazy_static! {
        //     static ref YAML_FRONT_MATTER_REGEXP: Regex = Regex::new(r"\A(---\s*\n.*?\n?)^((---|\.\.\.)\s*$\n?)").unwrap();
        // }

        let mut document_full_content = String::new();
        reader.read_to_string(&mut document_full_content)?;

        let YAML_FRONT_MATTER_REGEXP: Regex = Regex::new(r"^(?s)\s*---(.*)---(.*)$").unwrap();

        let captures = YAML_FRONT_MATTER_REGEXP.captures(&mut document_full_content)
            .expect("Invalid document format");

        println!("matches {}", captures.len());

        let content = captures.get(1).map_or("", |m| m.as_str());

        Ok(
            BlipDocument {
                metadata: HashMap::new(),
                content: content,
            })
    }
}


#[cfg(test)]
mod tests {
    use crate::blip_document::BlipDocument;

    #[test]
    fn test_parse_no_metadata() -> Result<(), String> {
        Ok(())
    }

    #[test]
    fn test_parse_no_content() -> Result<(), String> {
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<(), String> {
        let blip_file_content = "---\n
name: A name for the blip \n
---\n
The content here".as_bytes();

        let document = BlipDocument::parse(blip_file_content);

        // assert_eq!(document.unwrap().content, String::from("The content here"));
        assert_eq!(document.unwrap().content, String::from("content"));
        Ok(())
    }
}