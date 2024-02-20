mod ask;
mod config;
mod constants;
mod features;
mod filter;
mod train;

use ask::ask;
use config::Config;
use filter::filter;
use train::train;

use std::env::{
      args,
      Args,
};



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


fn main()
{
      let config = Config::new().expect("Failed to load config");

      let mut args = args();
      args.next(); // skip binary name

      let command =
            command(&mut args).expect("Failed to read command. Maybe path is not specified");
      match command {
            Command::Ask(path) => ask(path, &config),
            Command::Train(path) => train(path, &config),
            Command::Filter(path) => filter(path, &config).expect("ERROR: "),
      }
}
