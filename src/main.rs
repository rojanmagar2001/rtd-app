use std::u32;

use clap::{Parser, Subcommand, ValueEnum};
use rtd_app::{
    add_item, complete_item, delete_item, destroy_item, list_all, list_completed, list_deleted,
    list_uncompleted, restore_item, uncomplete_item,
};

#[derive(Parser, Debug)]
#[command(
    author = "Rojan Rana Magar <rozenmagar058@gmail.com>",
    version = "0.1.0",
    long_about = "A simple todo app write by Rust.\nYou can use it to make life pleasant or use it to learn the Rust language!"
)]
struct Args {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    Add {
        #[clap(short = 'n', long, value_name = "item-name")]
        name: String,
    },
    Complete {
        #[clap(short, long, value_name = "item-id")]
        id: u32,
    },
    Uncomplete {
        #[clap(short, long, value_name = "item-id")]
        id: u32,
    },
    Delete {
        #[clap(short, long, value_name = "item-id")]
        id: u32,
    },
    Restore {
        #[clap(short, long, value_name = "item-id")]
        id: u32,
    },
    Destroy {
        #[clap(short, long, value_name = "item-id")]
        id: u32,
    },
    List {
        #[clap(short = 't', long, value_name = "list-type")]
        list_type: Option<ListType>,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
enum ListType {
    /// All todo items
    All,

    /// All completed toto items [default]
    Completed,

    /// All uncompleted todo items
    Uncompleted,

    /// All deleted todo items
    Deleted,
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Command::Add { name } => match add_item(&name) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("Add '{name}' fail: {e}"),
        },
        Command::Complete { id } => match complete_item(id) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("Complete todo fail: {e}"),
        },
        Command::Uncomplete { id } => match uncomplete_item(id) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("Uncomplete todo fail: {e}"),
        },
        Command::Delete { id } => match delete_item(id) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("Delete todo fail: {e}"),
        },
        Command::Restore { id } => match restore_item(id) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("Restore todo fail: {e}"),
        },
        Command::Destroy { id } => match destroy_item(id) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("Destroy todo fail: {e}"),
        },
        Command::List { list_type } => {
            if let Some(list_type) = list_type {
                match list_type {
                    ListType::All => match list_all() {
                        Ok(s) => println!("{s}"),
                        Err(e) => eprint!("List all todos fail: {e}"),
                    },
                    ListType::Completed => match list_completed() {
                        Ok(s) => println!("{s}"),
                        Err(e) => eprint!("List all todos fail: {e}"),
                    },
                    ListType::Uncompleted => match list_uncompleted() {
                        Ok(s) => println!("{s}"),
                        Err(e) => eprint!("List all todos fail: {e}"),
                    },
                    ListType::Deleted => match list_deleted() {
                        Ok(s) => println!("{s}"),
                        Err(e) => eprint!("List all todos fail: {e}"),
                    },
                }
            } else {
                match list_all() {
                    Ok(s) => println!("{s}"),
                    Err(e) => eprint!("List all todos fail: {e}"),
                }
            }
        }
    }
}
