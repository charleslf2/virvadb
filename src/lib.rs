use ini::Ini;
use std::path::Path;

fn exist(path:&str)->bool{
    if Path::new(path).exists() == true{
        true
    }else{
        false
    }
    
}

/// Create a new database ; automaticly skip if the db already exist
pub fn new(path:&str){

    if exist(path) == false{
        let db=Ini::new();
        db.write_to_file(path).unwrap();
    }
}

/// insert new element in the database
pub fn insert(path:&str, data:&Vec<(&str, &str)>){

    if exist(path)==true{

        let mut db=Ini::load_from_file(path).unwrap();

        let id=db.len();

        for object in data.iter(){
            db.with_section(Some((id).to_string()))
                .set(object.0.to_string(), object.1.to_string());     
        }

        db.write_to_file(path).unwrap();

    }
    
}

/// return your database ; will panic if database don't exist
pub fn get_db(path:&str)->Ini{
    let db=Ini::load_from_file(path).unwrap(); 
    db    
}

/// update element in your database 
pub fn update(path:&str, data:&Vec<(&str, &str)>,pos:i32){

    if exist(path)==true{

        let mut db=Ini::load_from_file(path).unwrap(); 

        for object in data.iter(){

            let id=pos;

            db.with_section(Some((id).to_string()))
                .set(object.0.to_string(), object.1.to_string());
        }

        db.write_to_file(path).unwrap();

    }
}

/// delete element in your database in a given position ;automaticaly skip if the position did'int exist
pub fn delete(path:&str, pos:i32){

    if exist(path)==true{
        let mut db=Ini::load_from_file(path).unwrap(); 

        let db_ex=Ini::load_from_file(path).unwrap(); 

        for (sec, _prop) in db_ex.iter(){
            if Some(pos.to_string().as_str())==sec{
                db.delete(Some((pos).to_string()));
            }
        }

        db.write_to_file(path).unwrap();
    }
}

/// delete all element in the database
pub fn delete_all(path:&str){

    if exist(path)==true{

        let mut db=Ini::load_from_file(path).unwrap(); 

        let db_ex=Ini::load_from_file(path).unwrap(); 

        for i in 0..=db.len(){
            for (sec, _prop) in db_ex.iter(){
                if Some(i.to_string().as_str())==sec{
                    db.delete(Some(i.to_string()));
                }
            }
        }

        db.write_to_file(path).unwrap();
    }
}

