fn main() {
    let mut s1 = String::from("Rust"); //heap allocated string
    let r1 = &s1; //reference to s1
    print_string(r1);//s1 --> p1 //cloning it is not efficient
    let r2 = &mut s1; // line 4 <==> line 5 solve the problem because Rust knows you will use the variable after the print
    add_to_string(r2);
    println!("{s1}");
    let s2 = generate_string();
}

/*fn generate_string() -> &String { //dangling reference
    let s =  String::from("Ferris");
    &s

} //s is dropped*/ 
fn generate_string() -> String { 
    String::from("Ferris")
   

} 

fn add_to_string(p1: &mut String) {
    p1.push_str(" is awesome!");
    
}
fn print_string(p1: &String) { //referencing in the String
    println!("{p1}");
}