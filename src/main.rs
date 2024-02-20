mod ask;
mod config;
mod constants;
mod features;
mod train;

use ask::ask;
use config::Config;
use train::train;

use anyhow::Result;
use std::{
      env::{
            args,
            Args,
      },
      process::exit,
};



// eliquarre ask <texts.txt>
// eliquarre train <samples.jsonl>
// eliquarre filter <texts.txt>


#[derive(Debug)]
enum Command
{
      Ask(String),
      Train(String),
      Filter(String),
}


fn command(args: &mut Args) -> Option<Command>
{
      let command = args.next()?;
      let path = args.next()?;
      match command.as_str() {
            "ask" => Some(Command::Ask(path)),
            "train" => Some(Command::Train(path)),
            "filter" => Some(Command::Filter(path)),
            _ => None,
      }
}


fn error(message: &str)
{
      eprintln!("ERROR: {message}");
      exit(1);
}


fn filter(path: String) {}


fn main()
{
      let config = Config::new();
      if config.is_err() {
            error("Invalid config.");
      }

      let mut args = args();
      args.next(); // skip binary name

      let command = command(&mut args);
      if command.is_none() {
            error("Invalid command. Available commands: ask, train, filter");
      }

      match command.unwrap() {
            Command::Ask(path) => ask(path, &config.unwrap()),
            Command::Train(path) => train(path, &config.unwrap()),
            Command::Filter(path) => filter(path),
      }
}
