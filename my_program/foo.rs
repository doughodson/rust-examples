
mod bar;
pub use self::foo::Bar;

pub fn do_foo() {
   println!("Hi from foo!");
}
