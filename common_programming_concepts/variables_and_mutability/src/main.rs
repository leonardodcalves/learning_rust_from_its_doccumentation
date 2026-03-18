/*fn main() {
	let mut x = 5;
	println!("the value of x is: {x}");
	x = 6;
	println!("the value of x is: {x}");
}

fn main() {
	let x = 5;

	let x = x + 1;

	{
		let x = x * 2;
		println!("The value of x in the inner scope is: {x}");
	}

	println!("The value of x is: {x}");
}

fn main() {
    let mut spaces = "  ";
    spaces = spaces.len();
}

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
}

fn main() {
      let x = 2.0; // default floating-point type is f64

      let y: f32 = 3.0; // specified f32 length

      println!("x: {x}. y: {y}.");
}*/

// Numeric Operations

/*
fn main() {
    println!("Numeric Operations");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncatedu = -5 / 3;
    let truncatedf = -5.0 / 3.0; 

    // remainder
    let remainder = 43 % 5;

    println!("5 + 10 = {sum}.\n95.5 - 4.3 = {difference}.\n4 * 30 = {product}.\n\
              56.7 / 32.3 = {quotient}.\n-5 / 3 = {truncatedu} (integer).\n\
              -5.0 / 3.0 = {truncatedf} (float).\n43 % 5 = {remainder}");
}
*/

// The Boolean Type
/*
fn main() {
    let t = true;

    let f: bool = false; // explicit type annotation
}
*/

// The Character Type

fn main() {
    let c = 'z';
    let z: char = 'Z'; // explicit type annotation
    print!("c is {c}\nZ is {z}.\n");
}
