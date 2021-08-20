use clap::arg_enum;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "options")]
pub struct Opt {

    #[structopt(short, long)]
    debug: bool,

    #[structopt(possible_values = &SubCommand::variants(), case_insensitive = true)]
    pub subcmd: SubCommand, 
}

arg_enum! {
    #[derive(Debug)]
    pub enum SubCommand { 
        Add,
        Delete,
        Get,
        List,
        Update
    }
}


#[derive(Debug)]
pub struct Add {

}

#[derive(Debug)]
pub struct Delete {

}

#[derive(Debug)]
pub struct Get {

}

#[derive(Debug)]
pub struct List {

}

#[derive(Debug)]
pub struct Update {

}