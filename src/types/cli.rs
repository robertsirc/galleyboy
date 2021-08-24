
use clap::{ Clap };

#[derive(Clap)]
pub struct Opt {

    #[clap(short, long, about = "Debug flag")]
    debug: bool,

    #[clap(subcommand)]
    pub subcmd: SubCommand, 
}


#[derive(Clap)]
pub enum SubCommand { 
    #[clap(
        name = "add",
        about = "add"
    )]
    Add(Add),
    #[clap(
        name = "delete",
        about = "delete"
    )]
    Delete(Delete),
    #[clap(
        name = "get",
        about = "get"
    )]
    Get(Get),
    #[clap(
        name = "list",
        about = "list"
    )]
    List(List),
    #[clap(
        name = "update",
        about = "update"
    )]
    Update(Update)
}

#[derive(Clap, Debug)]
pub struct Add {
    pub repo_url: String,
}

#[derive(Clap, Debug)]
pub struct Delete {

}

#[derive(Clap, Debug)]
pub struct Get {
    
}

#[derive(Clap, Debug)]
pub struct List {

}

#[derive(Clap, Debug)]
pub struct Update {

}