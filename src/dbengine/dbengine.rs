use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::fs;
use std::io::BufReader;

#[derive(Clone)]
pub struct DbEngine{
    pub database: BTreeMap<String,String>,
}

impl DbEngine{
    pub fn new() ->  Self {
        println!("You got a new database");
        let mut newDb = DbEngine  {
            database: BTreeMap::new(),
        };
        load(&mut newDb.database);
        newDb
    }

    pub fn get(&self,key: &String) -> Result<String,()>{
        let result = self.database.get(key);
        save(&self.database);
        match result {
            Some(s) => Ok(s.clone()),
            None => Err(()),
        }
    }

    pub fn set(&mut self, key: &String, value: &String) -> Result<String,()>{
        let result = self.database.insert(key.clone(), value.clone());
        save(&self.database);
        match result {
            Some(s) => Ok(s),
            None => Err(()),
        }
    }

    pub fn delete(&mut self, key : &String) -> Result<String,()> {
        let result = self.database.remove(key);
        save(&self.database);
        match result {
            Some(s) => Ok(s),
            None => Err(()),
        }
    }

    pub fn scan(&self, key_min: &String, key_max: &String) -> DbEngine{
        let mut newDb = DbEngine::new();
        for (k,v) in self.database.range(key_min.clone()..key_max.clone()){
            println!("Find ({}----{})",k,v);
            newDb.database.insert(k.to_string(),v.to_string());
        }
        newDb
    }
}

fn save (database: &BTreeMap<String, String>) -> std::io::Result<()>{
    let mut file = File::create("log.txt")?;
    for (key, value) in database.iter() {
        file.write_all(key.as_bytes())?;
        file.write_all(b"\n")?;
        file.write_all(value.as_bytes())?;
        file.write_all(b"\n")?;
    }
    Ok(())
}

fn load(database: &mut BTreeMap<String, String>) -> std::io::Result<()> {
    let input = File::open("log.txt")?;
        let mut num = 0;
        let mut str_temp = String::new();
        let bufferead: BufReader<File> = BufReader::new(input);
        for line in bufferead.lines() {
            num+=1;
            let cur = match line {
                Ok(s) => s.to_string(),
                Err(_) => panic!("Error")
            };
            if(num==2) {
                database.insert(str_temp.clone(),cur);
                num = 0;
            } else {
                str_temp = cur;
            }
        }
        fs::remove_file("log.txt")?;
        File::create("log.txt")?;
        Ok(()) 
}