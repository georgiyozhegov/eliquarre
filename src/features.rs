use crate::config::Config;
use crate::constants::*;

use std::collections::HashMap;



type N = u32;


pub fn alphabetic(text: String) -> N
{
      text.chars().filter(is_alphabetic).collect::<Vec<_>>().len() as N
}


pub fn vowels(text: String) -> N
{
      text.chars().filter(is_vowel).collect::<Vec<_>>().len() as N
}


pub fn punctuations(text: String) -> N
{
      text.chars().filter(is_punctuation).collect::<Vec<_>>().len() as N
}


pub fn markings(text: String) -> N
{
      text.chars().filter(is_punctuation).collect::<Vec<_>>().len() as N
}


pub fn words(text: String) -> N
{
      text.split_whitespace().collect::<Vec<_>>().len() as N
}


pub fn paragraphs(text: String) -> N
{
      text.lines().collect::<Vec<_>>().len() as N
}


pub fn sentences(text: String) -> N
{
      let mut n = 0;
      let mut last_space = 0;
      for (i, c) in text.chars().enumerate() {
            if c == ' ' {
                  last_space = i;
            }
            else if c == '.' && i - last_space >= MIN_SENTENCE_LEN {
                  n += 1;
            }
      }
      n
}


pub fn features(text: String, config: &Config) -> HashMap<String, N>
{
      let mut features = HashMap::new();
      for feature in config.features.iter() {
            match feature.as_str() {
                  "words" => {
                        features.insert("words".to_string(), words(text.clone()));
                  },
                  "sentences" => {
                        features.insert("sentences".to_string(), sentences(text.clone()));
                  },
                  "paragraphs" => {
                        features.insert("paragraphs".to_string(), paragraphs(text.clone()));
                  },
                  "vowels" => {
                        features.insert("vowels".to_string(), vowels(text.clone()));
                  },
                  "alphabetic" => {
                        features.insert("alphabetic".to_string(), alphabetic(text.clone()));
                  },
                  "punctuations" => {
                        features.insert("punctuations".to_string(), punctuations(text.clone()));
                  },
                  "markings" => {
                        features.insert("markings".to_string(), markings(text.clone()));
                  },
                  _ => {},
            };
      }
      features
}
