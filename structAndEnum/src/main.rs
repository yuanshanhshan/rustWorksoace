#[derive(Debug)]
struct Person {
  name: String,
  age: u8,
}

// A uint struct
struct  Uint;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point{
  x: f32,
  y: f32,
}

//Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
  top_left: Point,
  bottom_right: Point,
}

fn main() {
  let name = String::from("Peter");
  let age = 27;
  let peter = Person { name, age};

  println!("{:?}", peter);
  
  let point: Point = Point { x: 10.3, y: 0.4};
  println!("point coordinates: ({}, {})", point.x, point.y); 
  
  let 
}
