

fn main() {
    let username = get_username(1);
    if let Some(name) = username {
        println!("Username: {name}");
    }
    
}

fn get_username(user_id: u32) -> Option<String> {
    let query =
        format!("GET username From users WHERE id={user_id}");
    //get usernames from the database
    let db_result = query_db(query);
    db_result.ok()

}



fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("Query string is empty"))
    } else {
        Ok(String::from("Ferris"))
    }
}