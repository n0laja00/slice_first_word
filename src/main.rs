//THeory: 
//Slice is how you reference a contiguous sequence of elements in a collection rather than the whole collection.
//Slices do not have ownership, this they are more akin to references. 
//Here's a problem. Find the first word of a example_string. A function takes the string and finds the first word. 
// Rather than finding a specific index and managing that, we should use String slicing! 

fn main() {
    let example_string = String::from("Hello world!"); 
    println!("Let's start with a String type example: {example_string}");

    //Example example_strings slices:
    let hello = &example_string[..5]; // Reference; [..5]=[0..5]
    println!("{hello}");
    let world = &example_string[6..]; // Reference; [6..]=[6..len]
    println!("{world}");

    //Works whether going with a partial or the whole array
    let word = first_word(&example_string[..6]);
    println!("{word}");
    let word = first_word(&example_string[..]);
    println!("{word}");

    //This also works, because it's equivelant to whole slices of String'
    let word = first_word(&example_string);
    println!("{word}");

    let example_string_literal = "Double hello world!";
    println!("Now, to string literals: {example_string_literal}");

    //Works with both.
    let word = first_word(&example_string_literal[..7]);
    println!("{word}");
    let word = first_word(&example_string_literal[..]);
    println!("{word}");

    //Because string literals "are" string slices already, it works without reference/slice syntax!
    let word = first_word(example_string_literal);
    println!("{word}");

    // Method 2 using Vectors
    let long_example_string = "Hello, what a nice day we're having! Oh my God!'";
    //Notice that we're using an str reference.
    let words: Vec<&str> = long_example_string.split_whitespace().collect(); //We're calling split_whitespace() method on the long_example_string. 
    //The iterator returns the substrings that are separated by a whitespace. Collect() method transforms the iterator (inserts results) into a collection, so the vector "words"
    // Printing for vectors is done so (:?). :#? for pretty print
    println!("{words:?}");
    println!("{words:#?}");
}

//a word is defined by the space that follows it. Let's find spaces.
//It's very tempting to search for a space and return the index of that space, but that wouldn't be very productive.
//We can find and slice the first word already by using a string slice.
//IT's also very tempting to add &String as the type as the type of parameter example_string, but &str serves a better more functional purpose. 
fn first_word(example_string: &str) -> &str {
    let bytes = example_string.as_bytes(); // make the received example_string reference an array of bytes.

    // We create an iterator to go over the array with iter method.
    // The enumerate wraps the result of iter and returns it as part of a tuple. The first element is the index. THe second is a reference to the element
    for (i, &item) in bytes.iter().enumerate() { 
        if item == b' ' {
            return &example_string[..i]; //return reference string slice; [..i]=[0..i]
        }
    }
    &example_string[..] //Return the slice returned from for loop.
}