use crate::lib::progress::{Progress};
use std::collections::VecDeque;
use rusqlite::{params, Connection, NO_PARAMS};

pub struct DBHanlde {
    conn: Connection,
    table_name: String,
}


// Currently every operation Works for single path operation correctly 
// except 'update' operation which is not tested Yet.
// However not sure when multiple paths are given.
// TODO Test Update Single operation
// TODO Test Multiple path operation
// 
// Currently Every opertation works for valid path given except update
impl DBHanlde {
    pub fn new(dbname: String, table_name: String) -> Self {
        let conn = Connection::open(dbname).expect("Failed to connect to db");
        conn.execute( 
            &format!("CREATE TABLE IF NOT EXISTS {} 
                (id INTEGER PRIMARY KEY AUTOINCREMENT, 
                 name TEXT NOT NULL UNIQUE,
                 description TEXT NOT NULL,
                 marked INTEGER NOT NULL CHECK (marked IN (0,1))
                 );", 
                &table_name), 
            NO_PARAMS
        ).expect("Faile dto create table");

        Self {  
            conn,
            table_name,
        }
    }

    // Should Include Error Result in function
    // Should Sanitize so that multiple names are prevented from Doing so 
    pub fn create (&self, mut path: VecDeque<String>, progress: Progress) -> rusqlite::Result<String>{
        self.conn.execute(
            &format!("INSERT INTO {} (name, description, marked) VALUES (?1, ?2, ?3)", self.table_name),
            params![
                progress.get_name(),// Name of the progress 
                progress.get_desc(),
                0
            ]
        )?;

        Ok(String::from("Succesfully Created progress"))

        //if path.len() == 0 { // No path is given which means insert to root directory
            //// Directly Insert to as a new progress
            //self.conn.execute(
                //&format!("INSERT INTO {} (name, content) VALUES (?1, ?2)", self.table_name),
                //params![
                    //progress.get_name(),// Name of the progress
                    //ron::to_string(&progress).unwrap() // Content of the progress
                //]
            //).expect("Failed to Create");
        //} else {
            //// Get first path Progress, which is base progress
            //let read_progress: String = self.conn.query_row(
                //&format!("SELECT content FROM {} WHERE name = ?1", self.table_name),
                    //params![path[0]], 
                    //|row|{
                        //let content: String = row.get(0)?;
                        //Ok(content)
                    //}
                //).expect("Failed to read progress by name");
            
            //let mut read_progress: Progress = ron::from_str(&read_progress).unwrap();
            //let name = path.pop_front().unwrap();
            //Progress::add_to_sub(Some(&mut read_progress), &mut path.clone(), &progress);
            //self.conn.execute(
                //&format!("UPDATE {} SET content = ?1 WHERE name = ?2", self.table_name), 
                //params![
                    //ron::to_string(&read_progress).unwrap(),
                    //name
                //]
            //);
            //// Add to progress
        //}
    }

    // For list and show command
    pub fn read (&self, mut path: VecDeque<String>) -> rusqlite::Result<String>{

        let mut stmt = self.conn.prepare(
            &format!("SELECT * FROM {}", self.table_name), 
        )?;

        let data_iter = stmt.query_map(NO_PARAMS, |row| {
            Ok( 
                Progress {
                    name: row.get(1)?,
                    description: row.get(2)?,
                    marked: row.get(3)?,
                    subs: None,
                }
            )
        })?;

        for data in data_iter {
            println!("{}", data?);
        }

        Ok(String::from("Succesfully read progress"))

        //if path.len() == 0 {
            //let mut stmt = self.conn.prepare(
                //&format!("SELECT * FROM {}", self.table_name), 
            //).unwrap();
            //let data_iter = stmt.query_map(NO_PARAMS, |row| {
                //Ok( 
                    //ProgressDB {
                        //name: row.get(1)?,
                        //content: row.get(2)?,
                    //}
                //)
            //}).unwrap();

            //for data in data_iter {
                //println!("{}", ron::from_str::<Progress>(&data.unwrap().content).expect("ERR"));
            //}
        //} else {
            //let base_progress = self.conn.query_row(
                //&format!("SELECT content FROM {} WHERE name = ?1", self.table_name), 
                //params![path[0]],
                //|row| {
                    //let ron_string: String = row.get(0).unwrap();
                    //Ok(ron_string)
                //}
            //).unwrap();
            //let mut target_progress: &Progress = &ron::from_str(&base_progress).unwrap();
            //path.pop_front();
            //for item in path {
                //target_progress = target_progress.get_subs().unwrap().get(&item).unwrap();
            //}
            //println!("{}", target_progress);
        //}
    }

    pub fn delete (&self, mut path: VecDeque<String>) -> rusqlite::Result<String>{

        self.conn.execute(
            &format!("DELETE FROM {} WHERE name = ?1", self.table_name), 
            params![path[0]]
        )?;

        Ok(String::from("Succesfully Delted data"))
        //if path.len() == 1 {
            //self.conn.execute(
                //&format!("DELETE FROM {} WHERE name = ?1", self.table_name), 
                //params![path[0]]
            //);
        //} else {
            //let base_progress = self.conn.query_row(
                //&format!("SELECT content FROM {} WHERE name = ?1", self.table_name), 
                //params![path[0]],
                //|row| {
                    //let ron_string: String = row.get(0).unwrap();
                    //Ok(ron_string)
                //}
            //).unwrap();
            //let mut target_progress: Progress = ron::from_str(&base_progress).unwrap();
            //let name = path.pop_front().unwrap();
            //Progress::delete_from_sub(Some(&mut target_progress), &mut path);
            //self.conn.execute(
                //&format!("UPDATE {} SET content = ?1 WHERE name = ?2", self.table_name), 
                //params![
                    //ron::to_string(&target_progress).unwrap(),
                    //name
                //]
            //);
        //}
    }

    pub fn update (&self, mut path: VecDeque<String>, name: String, description: String)  -> rusqlite::Result<String>{

        self.conn.execute(
            &format!("UPDATE {} SET name = ?1, description = ?2 WHERE name = ?3", self.table_name), 
            params![
                name,
                description,
                path[0],
            ]
        )?;

        Ok(String::from("Succesfully updated"))
        //if path.len() == 1 {
            //self.conn.execute(
                //&format!("DELETE FROM {} WHERE name = ?1", self.table_name), 
                //params![path[0]]
            //);
        //} else {
            //let base_progress = self.conn.query_row(
                //&format!("SELECT content FROM {} WHERE name = ?1", self.table_name), 
                //params![path[0]],
                //|row| {
                    //let ron_string: String = row.get(0).unwrap();
                    //Ok(ron_string)
                //}
            //).unwrap();
            //let mut target_progress: Progress = ron::from_str(&base_progress).unwrap();
            //let name = path.pop_front().unwrap();
            //Progress::delete_from_sub(Some(&mut target_progress), &mut path);
            //self.conn.execute(
                //&format!("UPDATE {} SET content = ?1 WHERE name = ?2", self.table_name), 
                //params![
                    //ron::to_string(&target_progress).unwrap(),
                    //name
                //]
            //);
        //}
    }
}

// SQLite Debugging
    //let conn = Connection::open("db/test2.db")?;

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

