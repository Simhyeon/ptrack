use clap::{App, Arg};
use std::collections::VecDeque;

pub enum OpType {
    Create,
    Read,
    Update,
    Delete,
    None,
}

pub struct Arguments {
    pub operation: OpType,
    pub path: VecDeque<String>,
}

impl Arguments {
    pub fn new() -> Self {
        let app = App::new("Progress Tracker")
            .version("0.1")
            .about("Progress Tracker with Hierarchy.")
            .author("Simon Creek");

        // Define the name command line option
        let list_option = Arg::with_name("list")
            .short('l')
            .long("list") // allow --name
            .about("List all progresses");

        let create_option = Arg::with_name("create")
            .short('c')
            .takes_value(true)
            .long("create")
            .about("Create new progress to path");

        let read_option = Arg::with_name("show")
            .short('s')
            .takes_value(true)
            .long("show")
            .about("Read new progress of path");

        let delete_option = Arg::with_name("delete")
            .short('d')
            .takes_value(true)
            .long("delete")
            .about("Delete the progress you want to set");

        let desc_option = Arg::with_name("description")
            .long("desc")
            .takes_value(true)
            .about("Take description rather than emtpy one.");

        // This is example template for some case that, like no internet or shit.
        //let age_option = Arg::with_name("age")
            //.short('a')
            //.long("age") // allow --name
            //.takes_value(true)
            //.about("age of the target")
            //.required(false);

        // now add in the argument we want to parse
        let app = app.arg(list_option)
                    .arg(read_option)
                    .arg(create_option)
                    .arg(delete_option)
                    .arg(desc_option);

        // extract the matches
        let matches = app.get_matches();


        let mut operation_type: OpType = OpType::None;
        let show_list = matches.is_present("list");
        let create_new = matches.is_present("create");
        let show_progress = matches.value_of("show");
        let delete_progress = matches.is_present("delete");
        let add_description = matches.value_of("description");

        if show_list {
            operation_type = OpType::Read;
        } else if create_new {
            operation_type = OpType::Create;
        } else if delete_progress {
            operation_type = OpType::Delete;
        }

        Self {  
            operation: operation_type,
            path: VecDeque::default(),
        }
    }
}
