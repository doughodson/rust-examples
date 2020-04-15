#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {

    // Enum with fields
   enum OptionalI32 {
      AnI32(i32),
      Nothing,
   }

   /////////////////////////
   // 3. Pattern matching //
   /////////////////////////

   let foo = OptionalI32::AnI32(1);
   match foo {
      OptionalI32::AnI32(n) => println!("it’s an i32: {}", n),
      OptionalI32::Nothing  => println!("it’s nothing!"),
   }

   // Advanced pattern matching
   struct FooBar { x: i32, y: OptionalI32 }
   let bar = FooBar { x: 15, y: OptionalI32::AnI32(32) };

   match bar {
      FooBar { x: 0, y: OptionalI32::AnI32(0) } =>
         println!("The numbers are zero!"),
      FooBar { x: n, y: OptionalI32::AnI32(m) } if n == m =>
         println!("The numbers are the same"),
      FooBar { x: n, y: OptionalI32::AnI32(m) } =>
         println!("Different numbers: {} {}", n, m),
      FooBar { x: _, y: OptionalI32::Nothing } =>
         println!("The second number is Nothing!"),
   }
}
