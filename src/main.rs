use rand::Rng;
use std::io;

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Player,
    Enemy,
    Treasure,
    Trap,
}

struct Player {
    x: usize,
    y: usize,
    health: i32,
    score: i32,
}

struct Game {
    map: [[Tile; 10]; 10],
    player: Player,
    treasure_count: i32,
    enemies: Vec<(usize, usize)>,
}

impl Game {
    fn new(score: i32, health: i32) -> Game {
        let number_of_enemies = 5;
        let treasure_count = 3;
        let mut map = [[Tile::Empty; 10]; 10];
        let mut rng = rand::thread_rng();
        let mut enemies = Vec::with_capacity(number_of_enemies);
        let player_x = rng.gen_range(0..10);
        let player_y = rng.gen_range(0..10);
        map[player_x][player_y] = Tile::Player;

        for _ in 0..number_of_enemies {
            let (x, y) = Self::random_empty(&map, &mut rng);
            map[x][y] = Tile::Enemy;
            enemies.push((x, y));
        }

        for _ in 0..treasure_count {
            let (x, y) = Self::random_empty(&map, &mut rng);
            map[x][y] = Tile::Treasure
        }

        for _ in 0..3 {
            let (x, y) = Self::random_empty(&map, &mut rng);
            map[x][y] = Tile::Trap
        }

        Game {
            map,
            treasure_count,
            enemies,
            player: Player {
                x: player_x,
                y: player_y,
                health,
                score,
            },
        }
    }

    fn random_empty(map: &[[Tile; 10]; 10], rng: &mut impl Rng) -> (usize, usize) {
        loop {
            let x = rng.gen_range(0..10);
            let y = rng.gen_range(0..10);
            if matches!(map[x][y], Tile::Empty) {
                return (x, y);
            }
        }
    }

    fn display(&self) {
        for row in self.map.iter() {
            for tile in row {
                match tile {
                    Tile::Player => print!("@ "),
                    Tile::Empty => print!(". "),
                    Tile::Enemy => print!(". "),
                    Tile::Treasure => print!("$ "),
                    Tile::Trap => print!(". "),
                }
            }
            println!();
        }

        print!(
            "Score: {} - Health {}\n",
            self.player.score, self.player.health
        )
    }

    fn move_player(&mut self, direction: char) -> bool {
        let (new_x, new_y) = match direction {
            'w' => (self.player.x - 1, self.player.y),
            's' => (self.player.x + 1, self.player.y),
            'a' => (self.player.x, self.player.y - 1),
            'd' => (self.player.x, self.player.y + 1),
            _ => return false,
        };

        if new_x >= 10 || new_y >= 10 {
            return false;
        }

        match self.map[new_x][new_y] {
            Tile::Empty => {}
            Tile::Trap => {
                self.player.health -= 20;
                println!("Hit a trap!");
                self.map[new_x][new_y] = Tile::Empty;
            }
            Tile::Enemy => {
                self.player.health -= 10;
                println!("Hit an Enemy!");
                self.map[new_x][new_y] = Tile::Empty;
                let enemy_index = self
                    .enemies
                    .iter()
                    .position(|coor| *coor == (new_x, new_y))
                    .unwrap();
                self.enemies.remove(enemy_index);
            }
            Tile::Treasure => {
                self.player.score += 10;
                println!("Found a treasure!");
                self.map[new_x][new_y] = Tile::Empty;
                self.treasure_count -= 1;
            }
            _ => return false,
        }
        self.map[self.player.x][self.player.y] = Tile::Empty;
        self.player.x = new_x;
        self.player.y = new_y;
        self.map[self.player.x][self.player.y] = Tile::Player;
        true
    }

    fn move_enemies(&mut self) {
        let mut rng = rand::thread_rng();
        let mut new_enemies = Vec::new();
        for _ in 0..self.enemies.len() {
            let (x, y) = Self::random_empty(&self.map, &mut rng);
            let (old_x, old_y) = self.enemies.pop().unwrap();
            self.map[old_x][old_y] = Tile::Empty;
            self.map[x][y] = Tile::Enemy;

            new_enemies.push((x, y));
        }

        self.enemies = new_enemies;
    }
}

fn main() {
    println!("Welcome to the dungeon!");
    let mut game = Game::new(0, 100);
    println!("WASD to move, q to quit");

    loop {
        game.display();
        if game.player.health <= 0 {
            println!("Game over!");
            break;
        }
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let direction = input.chars().next().unwrap();
        if direction == 'q' {
            println!(
                "Great job! Thanks for playing. Final stats -> Score: {} - Health: {}",
                game.player.score, game.player.health
            );
            break;
        }

        if !game.move_player(direction) {
            println!("Invalid move!");
        }

        if game.treasure_count == 0 {
            println!("You obtained all the treasures! New level starting...");
            game = Game::new(game.player.score, game.player.health);
        }

        if game.enemies.is_empty() {
            println!("All enemies have been killed!")
        }

        game.move_enemies()
    }
}
