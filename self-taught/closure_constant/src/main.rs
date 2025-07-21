
const BASE: i32 = 10;

fn nonnnto (g: i32 , t: i32) -> i32 {
    g * t
}

//closure functions can take things in the environment
// but what is inside the || cant be used outside of the {}
fn main () {
    let r = |portionmultiplier|{
    (BASE + 94) * portionmultiplier + nonnnto (8 , 6)
};
println! ("the result is : {}", r(22));
}

