
// unit structs have no size or associated data
// not very useful - sometimes used to represent errors
struct Dummy;

// tuple struct does not have field names
// useful with the number of fields is small
struct Color(u8, u8, u8);

struct Player {
   name: String,
   id: u8
}

fn main() {
   let value = Dummy;
   let white = Color(255, 255, 255);
   let player = Player{name: "Doug".to_string(), id: 100};

   println!("r:{} g:{} b:{}", white.0, white.1, white.2);
   println!("player name:{}, id:{}", player.name, player.id);
}

