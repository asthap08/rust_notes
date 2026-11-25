// in rust snake_case naming convention are followed i.e. my_function 



fn main() {
    //another_function(5);
    // let y: i32 = add( 5, 5);
    // println!("The value of y is: {y}");
    // //{
    //     let x = 3; //with semicolon it is an statment
    //     x + 1  //here no semicolon is used because we want to return the value of x + 1 from this block
    // };
    // println!("The value of y is: {y}");

   // is_even(5);

   let y: bool = is_even(5);
   println!("value of y is {y}");

   if y {
    println!("Fn returned true");
   }

}

//STATEMENT: The line which return any value is the statement like "let z=x+y"
//EXPRESSION: the line which does not return anything is a expression like "X + Y" is expression
// even a function call is an expression EXPRESSATIONS DO NOT HAVE SEMICOLON


// fn add(x: i32, y: i32) -> i32 {
//     x + y  //EXRESSION
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}"); // STATEMENT
// }

fn is_even(x: i32) -> bool {
    if x % 2 == 0 {
        return true;
    } 
    println!("the value is false");
    false
}

