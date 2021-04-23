use std::collections::HashMap;
use std::io::Error as IOError;
use std::error::Error;
use std::fs;

struct Todo {
    map: HashMap<String, bool>,
    filename: String
}

impl Todo {
    fn new(filename: String) -> Result<Todo, IOError> {
        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(&filename)?;
        match serde_json::from_reader(file) {
            Ok(map) => Ok(Todo { map, filename }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
                filename,
            }),
            Err(e) => panic!("An error occured: {}", e),
        }
    }

    fn save(self) -> Result<(), Box<dyn Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(self.filename)?;

        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn insert(&mut self, key: String) -> () {
        self.map.insert(key, true);
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    let mut todo = Todo::new(String::from("db.json")).expect("Initialisation of db failed");
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    }
}
