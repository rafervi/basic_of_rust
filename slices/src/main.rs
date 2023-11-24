fn main() {
    // Slices are references to a continuous sequence or
    // elements in a collection.
    
    let tweet = String::from(
        "This is my tweet and it's very long"
    );
    let trimmed_tweet = trim_tweet(&tweet); //why there is no error? -> because deref coersion
    let tweet2 = "This is my tweet and it's very long";
    let trimmed_tweet2 = trim_tweet(tweet2);
    println!("{trimmed_tweet}");
    println!("{trimmed_tweet2}");
    //String = growable, heap allocated string(UTF-8 encoded)
    //str = immutable sequence of UTF-8 bytes somewhere in memory(stack, heap, or static memory)
    let a = [1, 2, 3, 4, 5, 6];
    let a_slice = &a[..3];
    println!("{:?}",a_slice);
}
fn trim_tweet(tweet: &str) -> &str {
    &&tweet[..20]
}