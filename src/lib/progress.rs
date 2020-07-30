use std::fmt::{Display, Formatter, self};

#[derive(Clone, Default, Debug)]
pub struct Progress {
    pub name: String,
    pub description: String,
    pub marked: bool,
    pub subs: Option<Vec<Progress>>,
}

impl Progress {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            marked: false,
            subs: None,
        }
    }

    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn get_desc(&self) -> &str {
        return &self.description;
    }
}

impl Display for Progress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        // Put header before each level of progresses
        //for _ in  0..self.level {
            //write!(f, "    ").expect("Failed to print header characters");
        //}
        //if self.level != 0 {
            //write!(f, "|--").expect("Failed to print header characters");
        //}

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
        write!(f, "[{}]-[{}%]--> \"{}\"\n", self.name, 0 ,desc_display).expect("Failed to read name and percentage");
        if let Some(vec) = &self.subs {
            for value in vec {
                write!(f, "{}", value).expect("Failed to read sub progresses");
            }
        }

        // End writing and return formatter -> Maybe this line is not idiomatic and just totla hack
        write!(f, "")
    }
}
