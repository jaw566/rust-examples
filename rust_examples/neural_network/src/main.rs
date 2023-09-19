mod gates;

use crate::gates::*;

fn update(a: f64, a_deriv: f64, step: f64) -> f64 {
    return a + (step * a_deriv);  
}

fn derivative(fxy: f64, fxhy: f64, h: f64) -> f64 {
    return (fxhy - fxy) / h;
}

fn get_numerical_gradient(x: f64, y: f64, h: f64) -> (f64, f64) {
    let fxy = forward_multiply_gate(x, y);

    let xh = x + h;
    let fxh = forward_multiply_gate(xh, y);
    let dx = derivative(fxy, fxh, h);

    let yh = y + h;
    let fyh = forward_multiply_gate(x, yh);
    let dy = derivative(fxy, fyh, h);

    return (dx, dy);
}

fn main() {
    let mut x: f64 = -2.0; 
    let mut y: f64 = 3.0;
    let h: f64 = 0.0001;
    let step: f64 = 0.01;
    let mut newx: f64 = 0.0; 
    let mut newy: f64 = 0.0;
    let fxy = forward_multiply_gate(x, y);
    println!("original = {:.4}", fxy);
    
    /* Numerical gradient */
    let (mut dx, mut dy) = get_numerical_gradient(x, y, h);
    newx = update(x, dx, step);
    newy = update(y, dy, step);
    println!("x = {:.4}\ny = {:.4}", newx, newy);
    let new = forward_multiply_gate(newx, newy);
    println!("new = {:.4}", new);

    /* Analytic gradient (key insight: derivative of "f(x,y) = xy" w.r.t. x is y) */
    (dx, dy) = (y, x); 
    newx = update(x, dx, step);
    newy = update(y, dy, step);
    println!("x = {:.4}\ny = {:.4}", newx, newy);
    let new = forward_multiply_gate(newx, newy);
    println!("new = {:.4}", new);
     
    return ();
}
