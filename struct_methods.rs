
struct Player {
   name: String,
   id: u8
}

impl Player {
   fn create_player(name: &str) -> Player {
      Player { name: name.to_string(), id: 100}
   }
   fn set_id(&mut self, id: u8) { self.id = id; }
}

impl Player {
   fn set_name(&mut self, name: &str) { self.name = name.to_string(); }
}

fn main() {
   let mut player = Player::create_player("Doug");
   player.set_id(200);
   player.set_name("Bob");
   println!("player name:{}, id:{}", player.name, player.id);
}

