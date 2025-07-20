const GEORGE: i32 = 88;
const CLOSUREADDON: i32 = 13;
const VALUE_A: i32 = 5 ;
const VALUE_B: i32 = 19 ;

// a and b should not be confused with VALUE_A and VALUE_B
// they are just parameters of the function john and will get their values when the function is called
fn john (a : i32, b : i32) -> i32 {
    a + b / 3       //if keep decimals, use f64 instead of i32. and all values should be f64 and end with .0
}

// closure functions can take things in the environment
fn main () {
   let josh: Vec<i32>   = vec![14, 3, 19, 101, 0];
   let tom = |multiplier|{
        9 + (multiplier - 2 ) * CLOSUREADDON
   };
   // Vec<> is a dynamic array, can put any type of one-category data in it
   // josh.iter() means that we iterate/borrow the josh vector immutably
   // .map() means Modify All Pieces of the borrowed stuff 
   // !!.iter().map().filter().collect() is the common pattern
   // without .collect(), .map will not show the result
   // tom(*resemblejosh) means to take the value of the vector josh, dereference it, and then use in tom
   let results: Vec<_> = 
   josh.iter().map(|resemblejosh: &i32| resemblejosh * tom (*resemblejosh) - GEORGE + john(VALUE_A, VALUE_B)).collect();
   println!("{:?}", results); 

}


