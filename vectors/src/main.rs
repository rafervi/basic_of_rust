fn main() {
    let mut v = Vec::new(); //one way to create a vector
    v.push(String::from("One"));
    v.push(String::from("Two"));
    v.push(String::from("Three"));

    let v2 = vec![1, 2, 3];
    //index into vector
    let s = &v[0]; //can panic if we pass an invalid index
    //let s = v.remove(0); //remove

    let s = v.get(0); //safer  than use the bracket sintax --> not panic

    if let Some(e) = s {
        println!("{e}");
    }
    for s in &mut v {
        s.push_str("!");
    }
    for s in &v {
        println!("{s}"); //this time we borrow the string immutably
    }

    let mut v3 = vec![];

    for s in  v {
        v3.push(s);
        
    }//after for loop v is no longer valid

} //V is dropped


