
fn plus_one(i: i32) -> i32 {
   i + 1
}

fn main() {
   // without type inference
   let f: fn(i32) -> i32 = plus_one;
   // with type inference
   let g = plus_one;

   let six = g(5);
   println!("six is {}", six);

   // define z as a closure
   let z = |x:i32| -> i32 {x + 1};
   println!("closure {}", z(4));
}

