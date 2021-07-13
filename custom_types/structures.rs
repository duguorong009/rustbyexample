
use std::fmt;

#[derive(Debug)]
struct Person {
  name: String,
  age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, i32);

// A struct with two fields
struct Point {
  x: f32,
  y: f32,
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "\nx: {} y: {}", self.x, self.y)
  }
}

// Structs can be used as fields of another struct
#[allow(dead_code)]
struct Rectangle {
  // A rectangle can be specified by where the top left and bottom right
  // corners are in space.
  top_left: Point,
  bottom_right: Point,
}

impl fmt::Display for Rectangle {
  fn fmt ( &self, f: &mut fmt::Formatter) -> fmt::Result{
    write!(f, "\n{} {} \n{} {}", self.top_left.x, self.top_left.y, self.bottom_right.x, self.bottom_right.y)
  }
}

fn rect_area(rectangle: &Rectangle) -> f32 {
  let Rectangle { top_left: tl, bottom_right: br } = &*rectangle;
  (br.x - tl.x) * (tl.y - br.y)
}

fn square(pt: &Point, width: f32) -> Rectangle {
  let tl: Point = Point { y: pt.y + width, ..*pt };
  let br: Point = Point { x: pt.x + width, ..*pt };
  Rectangle { top_left: tl, bottom_right: br }
}

fn main() {
  // Create struct with field init shorthand
  let name = String::from("peter");
  let age = 27;
  let peter = Person { name, age };

  // Print debug struct
  println!("{:?}", peter);

  // Instantiate a point
  let point: Point = Point { x: 10.3, y: 0.4 };

  // Access the fields of the point
  println!("Point coords: ({}, {})", point.x, point.y);

  // Make a new point by using struct update syntax to use the fields of our 
  // other one
  let bottom_right = Point { x:5.2, ..point};

  // `bottom_right.y` will be the same as `point.y` because we used that field from `point`
  println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

  // Destructure the point using a 'let' binding
  let Point { x: top_edge, y: left_edge } = point;

  let _rectangle = Rectangle {
    // struct instantiation is an expression too
    top_left: Point { x: left_edge, y: top_edge },
    bottom_right: bottom_right,
  };

  // Instantiate a unit struct
  let _unit = Unit;

  // Instantiate a tuple struct
  let pair = Pair(1, 10);

  // Access the fields of a tuple struct
  println!("pair contains {:?} and {:?}", pair.0, pair.1);

  // Destructure a tuple struct
  let Pair(integer, decimal) = pair;

  println!("pair contains {:?} and {:?}", integer, decimal);

  println!("rect_area({}): {}", _rectangle, rect_area(&_rectangle));
  println!("square with Point {} & width {} : {}", point, 3.0, square(&point, 3.0));
}
