use ini::Ini;
use std::path::Path;

fn exist(path:&str)->bool{
    let created:bool=false;
    if Path::new(path).exists() == true{
        created=false;
    }else{
        created=true;
    }
    
    created
}

pub fn new(path:&str){

    if exist(path) == true{
        println!("the db already exist");
    }else{
        let db=Ini::new();
        db.write_to_file(path).unwrap();
    }
}

pub fn insert(path:&str, data:Vec<(&str, &str)>){
    if exist(path)==false{
        println!("the db don't exist yet !!!");
    }else{
        let mut db=Ini::load_from_file(path).unwrap();


        let mut id=db.len();

        for object in data.iter(){
            db.with_section(Some((id).to_string()))
                .set(object.0.to_string(), object.1.to_string());     
        }

        db.write_to_file(path).unwrap();

        id+=1;
    }
    
}

pub fn get_db(path:&str)->Ini{
   let db=Ini::load_from_file(path).unwrap(); 
   db
}

pub fn update(path:&str, data:Vec<(&str, &str)>,pos:i32){

    if exist(path)==false{
        println!("the db don't exist yet !!!");
    }else{

        let mut db=Ini::load_from_file(path).unwrap(); 

        for object in data.iter(){

            let id=pos;

            db.with_section(Some((id).to_string()))
                .set(object.0.to_string(), object.1.to_string());
        }

        db.write_to_file(path).unwrap();

    }
}

pub fn delete(path:&str, pos:i32){

    if exist(path)==false{
        println!("the db don't exist yet !!!")
    }else{
        let mut db=Ini::load_from_file(path).unwrap(); 

        db.delete(Some((pos).to_string()));

        db.write_to_file(path).unwrap();

    }
}

pub fn delete_all(path:&str){
    if exist(path)==false{
        println!("the db don't exist yet !!!")
    }else{
        let mut db=Ini::load_from_file(path).unwrap(); 
        
        for i in 0..db.len(){
            db.delete(Some((i).to_string()));
        }


        db.write_to_file(path).unwrap();

    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
