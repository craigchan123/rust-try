fn main() {
    // call the from function on string type to create a new string
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() adds a literal (world!) to a String

    println!("{s}"); 

}
