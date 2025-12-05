use std::io::{self};
use std::collections::HashMap;
use rand::Rng;
use rand::seq::SliceRandom;
use crate::room::Room;
use crate::player::Player;

#[derive(Debug)]
enum Event {
    None,
    Trap(i32),
    Luck(String),
}

pub struct Game {
    pub rooms: HashMap<String, Room>,
    pub player: Player,
    pub treasure_spawned: bool,
    pub key_spawned: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut rooms = HashMap::new();
        rooms.insert("Entrance".to_string(), Room::new("Entrance", "You awaken in a misty stone hall. The adventure begins!"));

        Game {
            rooms,
            player: Player::new("Entrance"),
            treasure_spawned: false,
            key_spawned: false,
        }
    }

    fn random_description() -> String {
        let descs = [
            "A crumbling stone chamber covered in moss.",
            "Torchlight flickers on ancient carvings.",
            "Cold wind howls through cracked walls.",
            "The air smells of old magic and dust.",
            "Echoes of distant water dripping.",
            "Strange runes glow faintly on the floor.",
            "Bones of forgotten adventurers lie scattered.",
            "A faint golden glow comes from the shadows...",
        ];
        let mut rng = rand::thread_rng();
        descs.choose(&mut rng).unwrap().to_string()
    }

    fn ensure_room_exists(&mut self, name: &str) {
        if self.rooms.contains_key(name) { return; }

        let mut rng = rand::thread_rng();
        let mut room = Room::new(name, &Self::random_description());
        room.generated = true;

        if !self.key_spawned && rng.gen_bool(0.07) {
            room.items.push("key".to_string());
            self.key_spawned = true;
            println!("A golden key gleams in the corner!");
        }
        if !self.treasure_spawned && rng.gen_bool(0.05) {
            room.items.push("treasure".to_string());
            self.treasure_spawned = true;
            println!("!!! LEGENDARY TREASURE DISCOVERED !!!");
        }
        if rng.gen_bool(0.25) {
            room.items.push("potion".to_string());
        }
        if rng.gen_bool(0.2) {
            let dirs = ["north", "south", "east", "west"];
            if let Some(dir) = dirs.choose(&mut rng) {
                room.locked_exits.push(dir.to_string());
            }
        }

        self.rooms.insert(name.to_string(), room);
    }

    pub fn run(&mut self) {
        println!("=== INFINITE RUSTY DUNGEON ====");
        println!("The world grows as you explore. There is no map. Only courage.");
        println!("Find the treasure somewhere in the infinite dark!\n");

        loop {
            let current_room_name = self.player.current_room.clone();
            let current_room = self.rooms.get(&current_room_name).unwrap();

            println!("=== {} (HP: {}) ===", current_room.name, self.player.health);
            println!("{}", current_room.description);
            if !current_room.items.is_empty() {
                println!("You see: {:?}", current_room.items);
            }
            println!("Exits: north, south, east, west");
            if !current_room.locked_exits.is_empty() {
                println!("Locked doors: {:?}", current_room.locked_exits);
            }

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("read error");
            let cmd = input.trim().to_lowercase();

            match cmd.as_str() {
                "q" | "quit" => break,
                "h" | "help" => {  // ← NEW: Help command!
                    println!("\n=== Commands ===");
                    println!("go <north/south/east/west> - Move");
                    println!("take <item> - Grab from room");
                    println!("use <item> - Use from inventory (e.g., use potion)");
                    println!("i/inventory - Check yer loot");
                    println!("h/help - This list");
                    println!("q/quit - Flee the dungeon\n");
                }
                "i" | "inventory" => println!("Inventory: {:?}", self.player.inventory),

                s if s.starts_with("go ") => {
                    let direction = s[3..].trim();
                    let valid_dirs = ["north", "south", "east", "west"];
                    if !valid_dirs.contains(&direction) {
                        println!("Go where?!");
                        continue;
                    }

                    let target_name = format!("{} {}", direction, current_room_name);

                    if current_room.locked_exits.contains(&direction.to_string())
                        && !self.player.inventory.contains(&"key".to_string()) {
                        println!("That door is locked tight!");
                        continue;
                    }

                    self.ensure_room_exists(&target_name);
                    self.player.current_room = target_name.clone();

                    if let Some(room) = self.rooms.get_mut(&current_room_name) {
                        room.exits.insert(direction.to_string(), target_name);
                    }

                    println!("You head {}...", direction);
                    self.trigger_random_event();
                }

                s if s.starts_with("take ") => {
                    let item = s[5..].trim().to_string();
                    if let Some(room) = self.rooms.get_mut(&current_room_name) {
                        if let Some(pos) = room.items.iter().position(|i| i == &item) {
                            room.items.swap_remove(pos);
                            self.player.pick_item(item.clone());
                            println!("Took the {}.", item);
                        } else {
                            println!("No {} here!", item);
                        }
                    }
                }

                // ← NEW: Use command!
                s if s.starts_with("use ") => {
                    let item = s[4..].trim().to_string();  // Note: "use " = 4 chars
                    let pos = self.player.inventory.iter().position(|i| i == &item);
                    match item.as_str() {
                        "potion" if pos.is_some() => {
                            self.player.inventory.swap_remove(pos.unwrap());
                            self.player.health = (self.player.health + 50).min(100);
                            println!("Potion guzzled! +50 HP (now {})", self.player.health);
                        }
                        "key" => println!("Keys unlock doors automatically with 'go'! No need ta 'use'."),
                        _ if pos.is_some() => println!("Can't use {} yet...", item),
                        _ => println!("No {} in inventory!", item),
                    }
                }

                _ => println!("Unknown? Type 'h' for help!"),
            }

            if self.player.inventory.contains(&"treasure".to_string()) {
                println!("\nTREASURE CLAIMED FROM THE INFINITE VOID!");
                println!("You are now a Rust Rogue-Like Legend!");
                break;
            }
        }
    }

    fn trigger_random_event(&mut self) {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.25) {
            let dmg = rng.gen_range(10..35);
            println!("TRAP! You lose {} HP!", dmg);
            self.player.take_damage(dmg);
        } else if rng.gen_bool(0.08) {
            println!("A potion rolls out from the shadows!");
            self.player.pick_item("potion".to_string());
        }
    }
}