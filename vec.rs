
fn main() {
   let mut numbers_vec: Vec<u8> = Vec::new();
   numbers_vec.push(1);
   numbers_vec.push(2);
   println!("numbers_vec:{:?}", numbers_vec);

   let mut vec_with_macro = vec![1];
   vec_with_macro.push(2);
   println!("vec_with_macro:{:?}", vec_with_macro);
}

