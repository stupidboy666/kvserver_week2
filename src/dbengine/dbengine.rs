use std::collections::BTreeMap;

#[derive(Clone)]
pub struct DbEngine{
    pub database: BTreeMap<String,String>,
}

impl DbEngine{
    pub fn new() ->  Self {
        println!("You got a new database");
        DbEngine  {
            database: BTreeMap::new(),
        }
    }

    pub fn get(&self,key: &String) -> Result<String,()>{
        let result = self.database.get(key);
        match result {
            Some(s) => Ok(s.clone()),
            None => Err(()),
        }
    }

    pub fn set(&mut self, key: &String, value: &String) -> Result<String,()>{
        let result = self.database.insert(key.clone(), value.clone());
        match result {
            Some(s) => Ok(s),
            None => Err(()),
        }
    }

    pub fn delete(&mut self, key : &String) -> Result<String,()> {
        let result = self.database.remove(key);
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