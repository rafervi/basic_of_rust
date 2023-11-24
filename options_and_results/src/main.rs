use std::ptr::null;

fn main() {
    let username = get_usernames(1);
    
    if let Some(name) = username {
        println!("Username: {name}");
    }
}

fn get_usernames(user_id: u32) -> Option<String> {
    // get username from database

    let db_result= String::from("Ferris");
    if user_id == 1 {
        Some(db_result)
    } else {
        None
    }
}


    
