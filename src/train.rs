use crate::{
      ask::Sample,
      error,
      Config,
};
use linfa::prelude::*;
use linfa_logistic::LogisticRegression;
use ndarray::{
      s,
      Array,
      Array2,
};

use serde_jsonlines::json_lines as read_jsonl;



pub fn train(path: String, config: &Config)
{
      let samples = {
            let samples = read_jsonl::<Sample, String>(path);
            if samples.is_err() {
                  error("Failed to read samples");
            }
            samples
                  .unwrap()
                  .filter(|s| s.is_ok())
                  .map(|s| s.unwrap())
                  .collect::<Vec<_>>()
      };

      let samples = samples
            .iter()
            .map(|s| {
                  let mut sample: Vec<f32> = Vec::new();
                  sample.extend(s.features().iter().map(|v| *v as f32).collect::<Vec<f32>>());
                  sample.push(s.valid as u32 as f32);
                  sample
            })
            .collect::<Vec<Vec<_>>>();

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


      let prediction = model.predict(&validation);
      let cm = prediction.confusion_matrix(&validation).unwrap();

      println!("Training info: ");
      println!("    Accuracy: {}\n    MCC: {}", cm.accuracy(), cm.mcc());
}
