
fn main() {
   let mut x = 5;
   loop {
      if x < 0 {
         break;
      }
      println!("{} more runs to go", x);
      x -= 1;
   }
}

