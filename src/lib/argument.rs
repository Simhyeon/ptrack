use clap::{App, Arg};
use std::collections::VecDeque;

#[derive(Debug)]
pub enum OpType {
    Create,
    Read,
    Update,
    Delete,
    None,
}

#[derive(Debug)]
pub struct Arguments {
    pub operation: OpType,
    pub path: VecDeque<String>,
    pub name: String,
    pub description: String,
}

impl Arguments {
    pub fn new() -> Self {
        let app = App::new("Progress Tracker")
            .version("0.1")
            .about("Progress Tracker with Hierarchy.")
            .author("Simon Creek");

        // Args options 
        let list_option = Arg::with_name("list")
            .short('l')
            .long("list") // allow --name
            .conflicts_with_all(&["create", "show", "delete", "name", "update","description", "path"])
            .about("List all progresses");

        let create_option = Arg::with_name("create")
            .short('c')
            .long("create")
            //.requires("path")
            .requires("name")
            .requires("description")
            .conflicts_with_all(&["list", "show", "delete", "update"])
            .about("Create new progress to path");

        let name_option = Arg::with_name("name")
            .long("name")
            .takes_value(true)
            .conflicts_with_all(&["list", "show", "delete"])
            .about("Sets name");

        let desc_option = Arg::with_name("description")
            .long("desc")
            .takes_value(true)
            .conflicts_with_all(&["list", "show", "delete"])
            .about("Sets description");

        let read_option = Arg::with_name("show")
            .short('s')
            .long("show")
            .requires("path")
            .conflicts_with_all(&["list", "create", "delete", "description", "update"])
            .about("Read new progress of path");

        let delete_option = Arg::with_name("delete")
            .short('d')
            .long("delete")
            .requires("path")
            .conflicts_with_all(&["list", "create" ,"show", "description", "update"])
            .about("Delete the progress you want to set");

        // Name 또는 desription 그리고 전부 받을 수 있도록 해야 한다. 
        let update_option = Arg::with_name("update")
            .short('u')
            .long("update")
            .requires("path")
            .requires("name")
            .requires("description")
            .conflicts_with_all(&["list", "create" ,"show", "delete"])
            .about("Update the progress");


        let path_option = Arg::with_name("path")
            .short('p')
            .long("path")
            .takes_value(true)
            .conflicts_with("list")
            .about("Specifies path that you want to modify");

        // Build arg option to app
        let app = app.arg(list_option)
                    .arg(read_option)
                    .arg(create_option)
                    .arg(name_option)
                    .arg(desc_option)
                    .arg(delete_option)
                    .arg(update_option)
                    .arg(path_option);

        // extract the matches
        let matches = app.get_matches();


        // Get arg values
        let mut operation_type: OpType = OpType::None;
        let show_list = matches.is_present("list");
        let create_new = matches.is_present("create");
        let show_progress = matches.is_present("show");
        let delete_progress = matches.is_present("delete");
        let update_progress = matches.is_present("update");

        let path = matches.values_of("path");
        let mut name:String = String::from("");
        let mut description:String = String::from("");

        if show_list {
            operation_type = OpType::Read;
        } else if create_new {
            operation_type = OpType::Create;
        } else if delete_progress {
            operation_type = OpType::Delete;
        } else if show_progress {
            operation_type = OpType::Read;
        } else if update_progress {
            operation_type = OpType::Update;
        }

        let mut path_string: VecDeque<String> =  VecDeque::new();
        if let Some(content) = path {
            for item in content {

                // root is reserved for root directory
                //if item == "root"{
                    //break;
                //}

                path_string.push_back(String::from(item));
            }
        }

        if let Some(content) = matches.value_of("name") {
            name = String::from(content);
        }

        if let Some(content) = matches.value_of("description") {
            description = String::from(content);
        }

        Self {  
            operation: operation_type,
            path: path_string,
            name,
            description,
        }
    }
}
