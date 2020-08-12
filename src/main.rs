use rand::Rng;

fn generate_dice(how_many: u32) -> Vec<u32> {
    let mut res = Vec::<u32>::new();

    for _ in 0..how_many {
        let random_die = rand::thread_rng().gen_range(1, 7);
        res.push(random_die);
    }

    return res;
}

#[derive(Debug)]
struct Player {
    money: u32,
    dice: Vec<u32>,
}

impl Player {
    pub fn new(default_money: u32) -> Player {
        Player {
            money: default_money,
            dice: generate_dice(0),
        }
    }

    pub fn roll_dice(&mut self) {
        for _ in 0..6 {
            let v = rand::thread_rng().gen_range(1, 7);
            self.dice.push(v);
        }
    }

    pub fn calculate_points(&self) -> u32 {
        let mut result: u32 = 0;

        let mut how_many: [u32; 7] = [0; 7];
        let mut most_common_count: u32 = 0;
        let mut most_common: u32 = 0;

        for d in &self.dice {
            how_many[*d as usize] += 1;
        }

        println!("How many: {:?}", how_many);

        for d in 0..7 {
            if how_many[d] * d as u32 > most_common_count * most_common {
                most_common = d as u32;
                most_common_count = how_many[d];
            }
        }

        let common_score = most_common * most_common_count;

        result = common_score;
        return result;
    }
}

fn main() {
    println!("intro...");

    let mut player = Player::new(100);
    let mut enemies = Vec::<Player>::new();
    let enemies_count = 3;
    for _ in 0..enemies_count {
        enemies.push(Player::new(50));
    }

    player.roll_dice();
    let points = player.calculate_points();

    println!("Player: {:?}", player);
    println!("Enemies: {:?}", enemies);

    println!("Players points: {}", points);
}
