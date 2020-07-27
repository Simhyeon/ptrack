pub mod lib;

// Local Library
use crate::lib::progress::{Progress, ProgressDB};
use crate::lib::argument::{Arguments, OpType};
use crate::lib::dbhandle::DBHanlde;
use std::collections::VecDeque;


fn main(){
    // Initiate Handler
    let db_handler = DBHanlde::new(
        String::from("db/test2.db"),
        String::from("Test")
        );
    let args = Arguments::new();

    println!("{:?}", args);

    match args.operation {
        OpType::Read => {
            println!("Read operation detected");
            db_handler.read(args.path);
        },

        OpType::Create => {
            println!("Create operation detected");
            db_handler.create(args.path, Progress::new(&args.name, &args.description));
        },

        OpType::Delete => {
            println!("Delete operation detected");
            db_handler.delete(args.path);
        },

        //OpType::Update => {
            //// TODO Change tis code to something really practial since this is not real thing
            //println!("Update operation detected");
            //let mut path = VecDeque::new();
            //path.push_back(String::from("Test2"));
            //db_handler.update(path, Progress::new("Test2", "DESC"));
        //},

        _ => println!("TODO"),
    }
}
