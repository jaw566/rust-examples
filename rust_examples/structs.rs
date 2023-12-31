/* There are 3 types of structs in Rust:
   Tuple structs (named tuples), classic 
   "C" structs, and unit structs.
*/ 

// classic "C" struct
struct Point {
    x: f64,
    y: i64
}

fn main()
{
    let point: Point = Point { x: 42.0, y: 42 };

    // print floating point and hex 
    println!("x={:.1}, y={:#04x}", point.x, point.y);

    let Point { x: left, y: right } = point;     

    println!("left={:.1}, right={:#04x}", left, right);

    return ();
}
