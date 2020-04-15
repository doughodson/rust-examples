enum Direction { N, E, S, W }

fn main() {
   let direction = Direction::N;

   match direction {
      Direction::N => println!("Moving north"),
      Direction::E => println!("Moving east"),
      Direction::S => println!("Moving south"),
      Direction::W => println!("Moving west")
   }
}

