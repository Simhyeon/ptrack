pub mod lib;

// Local Library
use crate::lib::progress::{Progress, ProgressDB};
use crate::lib::argument::{Arguments, OpType};
use crate::lib::dbhandle::DBHanlde;
use std::collections::VecDeque;


fn main(){
    //let mut new_progress = Progress::new("Test", "Lorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem Ipsum");

    //new_progress.add_sub(Progress::new("SubTest", "Test Description"));
    //new_progress.add_sub(Progress::new("SubTest2", "Test Description"));

    //new_progress.get_subs().unwrap().get_mut("SubTest")
        //.unwrap().add_sub(Progress::new("SubsubTest", "Test Description"));

    //new_progress.get_subs().unwrap().get_mut("SubTest")
        //.unwrap().add_sub(Progress::new("SubsubTest2", "Test Description"));

    //println!("{}", new_progress);

    // Add_To_Sub Example
    //let mut queue: VecDeque<String> = VecDeque::new();
    //queue.push_back(String::from("SubTest"));
    //queue.push_back(String::from("SubsubTest"));
    //let progress_ref = Progress::new("SubsubsubTest", "Hell Fucking Long shit");
    //Progress::add_to_sub(Some(&mut new_progress), &mut queue, &progress_ref);

    //println!("{}", new_progress);

    // Ron Debugging
    //let ron_string = ron::to_string(&new_progress).expect("ERR");
    //println!("{:?}", ron_string);
    //println!("{}", ron::from_str::<Progress>(&ron_string).expect("ERR"));

    //Ok(())

    // Communicate with embedded Squlite database
    //
    // Print out result if -
    // Create then result of creation
    // Read then show data
    // Update then show result of update
    // Delete then show result of deletion 
    //
    //


    // Initiate Handler
    let db_handler = DBHanlde::new(
        String::from("db/test2.db"),
        String::from("Test")
        );
    let args = Arguments::new();

    match args.operation {
        OpType::Read => {
            println!("Read operation detected");
            db_handler.read(VecDeque::new());
        },

        OpType::Create => {
            println!("Create operation detected");
            db_handler.create(VecDeque::new(), Progress::new("Test2", "DESC"));
        },

        OpType::Delete => {
            println!("Delete operation detected");
            let mut path = VecDeque::new();
            path.push_back(String::from("Test2"));
            db_handler.delete(path);
            db_handler.read(VecDeque::new());
            db_handler.create(VecDeque::new(), Progress::new("Test2", "DESC"));
        },

        OpType::Update => {
            // TODO Change tis code to something really practial since this is not real thing
            println!("Update operation detected");
            let mut path = VecDeque::new();
            path.push_back(String::from("Test2"));
            db_handler.update(path, Progress::new("Test2", "DESC"));
        },

        _ => println!("No args"),
    }
}
