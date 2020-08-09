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
    money: u64,
    dice: Vec<u32>,
}

impl Player {
    pub fn new(default_money: u64) -> Player {
        Player {
            money: default_money,
            dice: generate_dice(0),
        }
    }
}

fn main() {
    println!("intro...");

    let player = Player::new(100);
    let mut enemies = Vec::<Player>::new();
    let enemies_count = 3;
    for _ in 0..enemies_count {
        enemies.push(Player::new(50));
    }

    println!("Player: {:?}", player);
    println!("Enemies: {:?}", enemies);
}
