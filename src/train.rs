use crate::{
      ask::Sample,
      config::Config,
};

use linfa::prelude::*;
use linfa_logistic::{
      FittedLogisticRegression,
      LogisticRegression,
};
use ndarray::{
      s,
      Array,
      Array2,
};

use std::process::exit;

use serde_jsonlines::json_lines as read_jsonl;
use std::fs::File;
use std::io::{
      self,
      Write,
};



fn check_options(samples: &Vec<Vec<f32>>, config: &Config)
{
      print!("\x1b[2J");
      println!(
            "Config:\n    max_iterations: {}\n    split_ratio: {}\n    samples: {}",
            config.max_iterations,
            config.split_ratio,
            samples.len()
      );
      print!("Continue (y/n): ");
      io::stdout().flush().unwrap();
      match io::stdin().lines().next().unwrap().unwrap().as_str() {
            "y" | "yes" | "Y" => {},
            _ => exit(0),
      }
      println!();
}


fn train_(
      samples: Vec<Vec<f32>>,
      config: &Config,
) -> (FittedLogisticRegression<f32, &str>, ConfusionMatrix<&str>)
{
      let (rows, cols) = (samples.len(), samples[0].len());
      let flattened: Vec<f32> = samples.into_iter().flatten().collect();
      let samples: Array2<f32> = Array::from_shape_vec((rows, cols), flattened).unwrap();

      let num_features = config.features.len();
      let features = samples.slice(s![.., 0..num_features]).to_owned();
      let labels = samples.column(num_features).to_owned();

      let (train, validation) = Dataset::new(features, labels)
            .map_targets(|x| {
                  if *x > 0. {
                        "valid"
                  }
                  else {
                        "invalid"
                  }
            })
            .split_with_ratio(0.9);

      let model = LogisticRegression::default()
            .max_iterations(config.max_iterations.into())
            .fit(&train)
            .unwrap();
      let cm = model.predict(&validation).confusion_matrix(&validation).unwrap();
      (model, cm)
}


pub fn train(path: String, config: &Config)
{
      let samples = {
            let samples = read_jsonl::<Sample, String>(path).expect("Failed to read samples.jsonl");
            samples.filter(|s| s.is_ok()).map(|s| s.unwrap()).collect::<Vec<_>>()
      };
      let samples = samples
            .iter()
            .map(|s| {
                  let mut sample: Vec<f32> = Vec::new();
                  sample.extend(s.features().iter().map(|v| *v as f32).collect::<Vec<f32>>());
                  sample.push(s.valid.unwrap() as u32 as f32);
                  sample
            })
            .collect::<Vec<Vec<_>>>();

      check_options(&samples, config);

      let (model, cm) = train_(samples, config);

      println!("Training info: ");
      println!("    Accuracy: {}\n    MCC: {}\n", cm.accuracy(), cm.mcc());

      let file = File::create("model.json").unwrap();
      serde_json::to_writer(file, &model).unwrap();

      println!("Model saved to model.json");
}
