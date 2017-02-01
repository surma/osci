//use std::io;

trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    w: f64,
    h: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        return self.w * self.h;
    }
}

struct Circle {
    r: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        return std::f64::consts::PI * self.r * self.r;
    }
}

fn is_bigger_shape_generics<T, U>(a: &T, b: &U) -> bool
    where T: Shape,
          U: Shape
{
    return a.area() > b.area();
}

fn is_bigger_shape_type(a: &Shape, b: &Shape) -> bool {
    return a.area() > b.area();
}

fn main() {
    let c = Circle { r: 1.0f64 };
    let r = Rectangle {
        w: 1.0f64,
        h: 1.0f64,
    };
    println!("c = {}, r = {}", c.area(), r.area());
    println!("c > r = {}", is_bigger_shape_generics(&c, &r));
    println!("c > r = {}",
             is_bigger_shape_type(&c as &Shape, &r as &Shape));

    let greater_than_forty_two = (0..100).find(|x| *x > 42);

    match greater_than_forty_two {
        Some(_) => println!("Found a match!"),
        None => println!("No match found :("),
    }
}
