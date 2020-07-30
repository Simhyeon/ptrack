pub mod lib;

// Local Library
use crate::lib::progress::{Progress};
use crate::lib::argument::{Arguments, OpType};
use crate::lib::dbhandle::DBHanlde;

fn main() -> rusqlite::Result<()>{
    // Initiate Handler
    let db_handler = DBHanlde::new(
        String::from("db/test.db"),
        String::from("Test")
        );
    let args = Arguments::new();

    println!("{:?}", args);

    match args.operation {
        OpType::Read => {
            println!("Read operation detected");
            println!("{}", db_handler.read(args.path)?);
        },

        OpType::Create => {
            println!("Create operation detected");
            println!("{}", db_handler.create(args.path, Progress::new(&args.name, &args.description))?);
        },

        OpType::Delete => {
            println!("Delete operation detected");
            println!("{}", db_handler.delete(args.path)?);
        },

        OpType::Update => {
            // TODO Change tis code to something really practial since this is not real thing
            println!("Update operation detected");
            println!("{}", db_handler.update(args.path, args.name, args.description)?);
        },

        _ => println!("TODO"),
    }

    Ok(())
}
