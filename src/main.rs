pub mod lib;

use lib::Progress;
use clap::{App, Arg};

pub struct Arguments { }

impl Arguments {
    // These are some basic template for clap functions
    fn get_args(&mut self) { // -> This should be changed to new value that return Arguments struct
        let app = App::new("App name")
            .version("1.0")
            .about("App Description")
            .author("Simon Creek");

        // Define the name command line option
        let name_option = Arg::with_name("name")
            .short('n')
            .long("name") // allow --name
            .takes_value(true)
            .about("Hello target")
            .required(true);

        let age_option = Arg::with_name("age")
            .short('a')
            .long("age") // allow --name
            .takes_value(true)
            .about("age of the target")
            .required(false);

        // now add in the argument we want to parse
        let app = app.arg(name_option).arg(age_option);

        // extract the matches
        let matches = app.get_matches();

        // Extract the actual name
        let name = matches.value_of("name")
            .expect("This can't be None, we said it was required");
        let age = matches.value_of("age");

        if let Some(value) = age {
            println!("Hello, {} who is {}'s old!", name, value);
        } else {
            println!("Hello, {}!", name);
        }
    }
}

fn main() {
    let mut new_progress = Progress::new("Test", "Lorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem Ipsum");

    println!("{}", new_progress);

    new_progress.add_sub(Progress::new("SubTest", "Test Description"));
    new_progress.add_sub(Progress::new("SubTest2", "Test Description"));

    println!("{}", new_progress);

    new_progress.get_subs().unwrap().get_mut("SubTest")
        .unwrap().add_sub(Progress::new("SubsubTest", "Test Description"));

    new_progress.get_subs().unwrap().get_mut("SubTest")
        .unwrap().add_sub(Progress::new("SubsubTest2", "Test Description"));

    println!("{}", new_progress);

    // Communicate with embedded Squlite database
    //
    // Print out result if -
    // Create then result of creation
    // Read then show data
    // Update then show result of update
    // Delete then show result of deletion 
}
