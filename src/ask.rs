use crate::{
      error,
      features::features,
      Config,
};
use serde::{
      Deserialize,
      Serialize,
};
use serde_jsonlines::write_json_lines as write_jsonl;

use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::fs;
use std::io::{
      self,
      BufRead,
      Write,
};



#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Sample
{
      pub features: HashMap<String, u32>,
      pub valid: bool,
}

impl Sample
{
      pub fn new(text: String, valid: bool, config: &Config) -> Self
      {
            Self {
                  features: features(text, config),
                  valid,
            }
      }

      pub fn features(&self) -> Vec<u32>
      {
            let binding = self.features.clone();
            let mut features = binding.iter().collect::<Vec<_>>();
            features.sort_by(|a, b| a.0.cmp(b.0));
            features.iter().map(|(_k, v)| **v).collect()
      }
}


fn is_valid(text: &String, step: u32, steps: u32) -> bool
{
      print!("\x1b[2J");
      println!("{text}\n");
      print!("[{step}/{steps}] Is this text valid (y/n): ");
      io::stdout().flush().unwrap();
      match io::stdin().lock().lines().next().unwrap().unwrap().as_str() {
            "y" | "yes" | "Y" => true,
            _ => false,
      }
}


fn generate(file: String, config: &Config) -> Vec<Sample>
{
      let mut samples = Vec::new();
      let texts = file.split(&config.split_token).collect::<Vec<_>>();
      let step_size = texts.len() / config.samples as usize;
      for step in (0..texts.len()).step_by(step_size) {
            let text = {
                  let mut rng = rand::thread_rng();
                  texts.choose(&mut rng).unwrap()
            };
            samples.push(Sample::new(
                  text.to_string(),
                  is_valid(&text.to_string(), (step / step_size) as u32, config.samples),
                  config,
            ));
      }
      samples
}


pub fn ask(path: String, config: &Config)
{
      let file = {
            let file = fs::read_to_string(&path);
            if file.is_err() {
                  error(format!("Failed to read {}", path).as_str())
            }
            file.unwrap()
      };

      let samples = generate(file, config);

      if write_jsonl("samples.jsonl", samples).is_err() {
            error("Failed to write samples");
      }
}
