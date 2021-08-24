use clap::Clap;
use types::cli::{Opt, SubCommand};


mod commands;
mod types;

fn main() {
    let opt = Opt::parse();

    match opt.subcmd {
        SubCommand::Add(o) => commands::add::add(&o),
        SubCommand::Delete(o) => commands::delete::delete(&o),
        SubCommand::Get(o) => commands::get::get(&o),
        SubCommand::List(o) => commands::list::list(&o),
        SubCommand::Update(o) => commands::update::update(&o),
    }
    
}
