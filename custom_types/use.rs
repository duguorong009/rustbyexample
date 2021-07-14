// an attribute to hide warnings for unused code.
#![allow(dead_code)]

enum Status {
  Rich,
  Poor,
}

enum Work {
  Civilian,
  Soldier,
}

fn main() {
  // Explicitl `use` each name so they are available without 
  // manual scoping.
  use crate::Status::{ Poor, Rich };
  // automatically `use` each name inside `Work`
  use crate::Work::*;

  // Equivalent to `Status::Poor`
  let status = Poor;
  // Equivalent to `Work::Civilian`
  let work = Civilian;

  match status {
    // Note the lack of scoping because of the explicit `use` above
    Rich => println!("The rich have lots of memory"),
    Poor => println!("The Poor have no money..."),
  }

  match work {
    Civilian => println!("Civilians work!"),
    Soldier => println!("Soldiers fight"),
  }

}
