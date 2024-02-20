use serde::Deserialize;
use serde_json::from_reader;
use std::collections::HashSet;

use anyhow::Result;
use std::fs::File;
use std::io::BufReader;



#[derive(Debug)]
#[derive(Deserialize)]
pub struct Config
{
      #[serde(default)]
      pub features: HashSet<String>,
      #[serde(default)]
      pub samples: u32,
      #[serde(default)]
      pub max_iterations: u32,
      #[serde(default)]
      pub split_token: String,
}

impl Default for Config
{
      fn default() -> Self
      {
            Self {
                  features: HashSet::from([
                        "words".to_string(),
                        "sentences".to_string(),
                        "paragraphs".to_string(),
                        "vowels".to_string(),
                        "alphabetic".to_string(),
                        "punctuations".to_string(),
                        "markings".to_string(),
                  ]),
                  samples: 30,
                  max_iterations: 90,
                  split_token: String::from("<[SPLIT]>"),
            }
      }
}

impl Config
{
      pub fn new() -> Result<Self>
      {
            let file = File::open("eliquarre.json")?;
            let reader = BufReader::new(file);
            Ok(from_reader(reader)?)
      }
}
