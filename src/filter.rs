use crate::{
      ask::Sample,
      config::Config,
};
use anyhow::Result;
use linfa::prelude::*;
use linfa_logistic::*;
use ndarray::Array1;

use serde_json::from_str;
use std::fs;


pub fn filter(path: String, config: &Config) -> Result<()>
{
      let file = fs::read_to_string(path)?;

      let model = fs::read_to_string("model.json")?;
      let model: FittedLogisticRegression<f32, &str> = from_str(&model)?;
      println!("{model:?}");

      let mut texts = file.split(&config.split_token).collect::<Vec<_>>();
      for (i, text) in texts.iter().enumerate() {
            let sample = Sample::new(text.to_string(), None, config);
            let features = Array1::from_shape_vec(
                  config.features.len(),
                  sample.features().iter().map(|v| *v as f32).collect(),
            )?;
            let features = Dataset::new(features);
            if false {
                  // texts.remove(i);
            }
      }
      Ok(())
}
