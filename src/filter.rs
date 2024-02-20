use crate::{
      ask::Sample,
      config::Config,
};
use anyhow::Result;
use linfa::prelude::*;
use linfa_logistic::FittedLogisticRegression;
use ndarray::Array1;
use serde::{
      Deserialize,
      Serialize,
};
use serde_json::from_str;
use std::fs;


pub fn filter(path: String, config: &Config) -> Result<()>
{
      let file = fs::read_to_string(path)?;

      let model: FittedLogisticRegression<f32, &str> =
            from_str(&fs::read_to_string("model.json")?)?;
      println!("{model:?}");

      let mut texts = file.split(&config.split_token).collect::<Vec<_>>();
      for (i, text) in texts.iter().enumerate() {
            let sample = Sample::new(text.to_string(), None, config);
            let prediction = model.predict(sample.features());
            if false {
                  texts.remove(i);
            }
      }
      Ok(())
}
