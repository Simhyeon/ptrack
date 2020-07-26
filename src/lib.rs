use std::fmt::{Display, Formatter, self};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use std::collections::VecDeque;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Progress {
    name: String,
    description: String,
    percentage: i32,
    level: i32,
    subs: Option<BTreeMap<String, Progress>>,
}

#[derive(Clone, Default, Debug)]
pub struct ProgressDB {
    pub name: String,
    pub content: String,
}

impl Progress {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            percentage: 0,
            level: 0,
            subs: None,
        }
    }

    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    // 이걸 좀 더 체계적으로 변경해야 겠지 싶다. 
    pub fn add_sub(&mut self, mut progress: Progress) {
        // Make subs Some if no sub progress exists
        if let None = self.subs {
            self.subs.replace(BTreeMap::new());
        } 

        progress.level = self.level + 1; 
        let mut new = self.subs.take().unwrap();
        new.insert(progress.name.clone(), progress);
        self.subs.replace(new);
    }
}

impl<'a> Progress {
    pub fn get_subs(&mut self) -> Option<&mut BTreeMap<String, Progress>> {
        //println!("GETTING SUBS");
        //println!("{:?}", self.subs);
        return self.subs.as_mut();
    }

    pub fn add_to_sub(progress: Option<&'a mut Progress>, path : &'a mut VecDeque<String>, new: &'a Progress) -> Option<&'a mut Progress> {
        if let Some(handled) = progress {
            let new_progress: &mut Progress;
            if path.len() == 0 {
                handled.add_sub(new.clone());
                None
            } else {
                new_progress = handled.get_subs().expect("ERR 1 WUT").get_mut(&path[0]).expect("ERR 2");
                path.pop_front();
                Progress::add_to_sub(Some(new_progress), path, new)
            }
        } else{
            None
        }
    }

    //pub fn get_progress(progress: &'a mut Progress, path : Vec<&str>) -> Option<&'a mut Progress> {
        //if let Some(btreemap) = progress.get_subs() {
            //Some(btreemap.get(path[0]).unwrap())
        //} else {
            //None
        //}
    //}

    //pub fn get_progress(&mut self, path : Vec<&str>) -> Option<&mut Progress> {
        //let mut current_value: &mut Progress = self;
        //self.get_progress(current_value = current_value.subs.as_ref().unwrap().get(item).as_mut().unwrap())
    //}
}

impl Display for Progress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        // Put header before each level of progresses
        for _ in  0..self.level {
            write!(f, "    ").expect("Failed to print header characters");
        }
        if self.level != 0 {
            write!(f, "|--").expect("Failed to print header characters");
        }

        // Cut off description if too long Hard cap as 20 characters.
        // Should be decided by the width of terminal
        // Maybe I should find a way to get terminal width
        // Maybe calling min for terminal size and pre configured value so that player can
        // Configure to set maxium width which is yet trivial also
        // Actually I tried multiple crates and yet didn't succeed... why?
        let term_size: usize;
        term_size = 20;

        let desc_display: String;
        if self.description.len() > term_size{
            desc_display = [self.description.get(0..term_size).unwrap(), "..."].join("");
        } else {
            desc_display = self.description.clone();
        }

        // Write all informations formatter
        // recursion occurs if more than one subprogress exist
        write!(f, "[{}]-[{}%]--> \"{}\"\n", self.name , self.percentage, desc_display).expect("Failed to read name and percentage");
        if let Some(map) = &self.subs {
            for value in map.values() {
                write!(f, "{}", value).expect("Failed to read sub progresses");
            }
        }

        // End writing and return formatter -> Maybe this line is not idiomatic and just totla hack
        write!(f, "")
    }
}
