fn main() {
  // variables can be type annotated
  let logical: bool = true;

  let a_float: f64 = 1.0; // regular annotation
  let an_integer = 5i32; // suffix annotation

  // or a default will be used
  let default_float = 3.0; // f64
  let default_integer = 7; // i32

  // a type can also be infered from context
  let mut inferred_type = 12; // type i64 is inferred from another line
  inferred_type = 294859234;

  // a mutable variable's value can be changed. 
  let mut mutable = 12; // mutable i32
  mutable = 32;

  // Error! the type of a variable can't be changed.
  mutable = true;

  // variables can be overwritten with shadowing.
  let mutable = true;
}
