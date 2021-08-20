use structopt::StructOpt;
use types::cli::{Opt, SubCommand};

mod commands;
mod types;

fn main() {
    let opt = Opt::from_args();

    match opt.subcmd {
        SubCommand::Add => todo!(),
        SubCommand::Delete => todo!(),
        SubCommand::Get => todo!(),
        SubCommand::List => todo!(),
        SubCommand::Update => todo!(),
    }
    
}
