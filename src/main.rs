pub mod lib;
use ron;
use rusqlite::{params,Connection, Result, NO_PARAMS};
use std::collections::VecDeque;

use lib::{Progress, ProgressDB};
use clap::{App, Arg};

pub struct Arguments { }

impl Arguments {
    // These are some basic template for clap functions
    fn get_args(&mut self) { // -> This should be changed to new value that return Arguments struct
        let app = App::new("Progress Tracker")
            .version("0.1")
            .about("Progress Tracker with Hierarchy.")
            .author("Simon Creek");

        // Define the name command line option
        let name_option = Arg::with_name("list")
            .long("list") // allow --name
            .about("List all progresses");

        // This is example template for some case that, like no internet or shit.
        //let age_option = Arg::with_name("age")
            //.short('a')
            //.long("age") // allow --name
            //.takes_value(true)
            //.about("age of the target")
            //.required(false);

        // now add in the argument we want to parse
        let app = app.arg(name_option);

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

fn main() -> Result<()>{
    let mut new_progress = Progress::new("Test", "Lorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem IpsumLorem Ipsum");

    new_progress.add_sub(Progress::new("SubTest", "Test Description"));
    new_progress.add_sub(Progress::new("SubTest2", "Test Description"));

    new_progress.get_subs().unwrap().get_mut("SubTest")
        .unwrap().add_sub(Progress::new("SubsubTest", "Test Description"));

    new_progress.get_subs().unwrap().get_mut("SubTest")
        .unwrap().add_sub(Progress::new("SubsubTest2", "Test Description"));

    println!("{}", new_progress);

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

    // SQLite Debugging
    let conn = Connection::open("db/test2.db")?;

    // Create Table------------------------------
    //conn.execute(
        //"CREATE TABLE IF NOT EXISTS test (
             //id integer primary key,
             //name text not null unique,
             //content text not null unique
         //)",
        //NO_PARAMS,
    //)?;
    //-------------------------------------------

    // Insert new Value--------------------------
    //conn.execute(
        //"INSERT INTO test (name, content) VALUES (?1, ?2)",
        //params![new_progress.get_name(), ron_string],
    //)?;
    //-------------------------------------------
    
    // SELECT EVERYTHING AND SHOW------------------------
    // Kinda Read Operation
    //let mut stmt = conn.prepare("SELECT * FROM test")?;
    //let data_iter = stmt.query_map(params![], |row| {
        //Ok( 
            //ProgressDB {
                //name: row.get(1)?,
                //content: row.get(2)?,
            //}
        //)
    //})?;
    //for data in data_iter {
        //println!("{}", ron::from_str::<Progress>(&data?.content).expect("ERR"));
    //}
    //----------------------------------------------------

    // Only print names 
    //let mut stmt = conn.prepare("SELECT name FROM test")?;
    //let data_iter = stmt.query_map(NO_PARAMS, |row|{
        //let check: String = row.get(0)?;
        //Ok (
            //check
        //)
    //})?;
    //for data in data_iter {
        //println!("{}", data.unwrap());
    //}

    // Delete
    //conn.execute("DELETE FROM test WHERE name = 'Test'", NO_PARAMS)?;

    // Update
    // Get name input
    // Get value and deserialize find value if exists modify if not return error
    // Update value 
    // Update value as new thing
    //let target: String = String::from("Test");
    //let mut queue: VecDeque<String> = VecDeque::new();
    //queue.push_back(String::from("SubTest"));
    //queue.push_back(String::from("SubsubTest"));
    //let updater_progress = Progress::new("Tttest", "This is Test Description");

    //let mut read_progress = ron::from_str::<Progress>(
        //&conn.query_row("SELECT content FROM test WHERE name = ?1", params![target], |row|{
            //let res: String = row.get(0)?;
            //Ok(res)
        //})?
    //).expect("Could not find target row");

    //// Add New Progress under path
    //Progress::add_to_sub(Some(&mut read_progress), &mut queue, &updater_progress);

    //// Make new Ron type variable
    //let ron_string = ron::to_string(&read_progress).expect("ERR");

    //// Really Update to Database
    //conn.execute(
        //"UPDATE test SET content = ?1 WHERE name = ?2",
        //params![ron_string, target],
    //)?;


    Ok(())
    // Communicate with embedded Squlite database
    //
    // Print out result if -
    // Create then result of creation
    // Read then show data
    // Update then show result of update
    // Delete then show result of deletion 
}
