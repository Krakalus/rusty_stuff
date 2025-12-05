#[derive(Debug)]
pub struct Player {
    pub current_room: String,
    pub inventory: Vec<String>,
    pub health: i32,  // Starts at 100, traps can hurt!
}

impl Player {
    pub fn new(start_room: &str) -> Self {
        Player {
            current_room: start_room.to_string(),
            inventory: Vec::new(),
            health: 100,
        }
    }

    pub fn pick_item(&mut self, item: String) {
        self.inventory.push(item);
    }

    pub fn take_damage(&mut self, dmg: i32) {
        self.health -= dmg;
        if self.health <= 0 {
            println!("Oof! Ye perished. Game over!");
            std::process::exit(0);  // Ends the program
        } else {
            println!("Health now: {}", self.health);
        }
    }
}