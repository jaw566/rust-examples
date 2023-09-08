/* There are 3 types of structs in Rust:
   Tuple structs (named tuples), classic 
   "C" structs, and unit structs.
*/ 

// classic "C" struct
struct Point {
    x: f64,
    y: f64
}

fn main()
{
    let point = Point { x: 42.0, y: 42.0 };

    println!("x={}, y={}", point.x, point.y);

    return ();
}
