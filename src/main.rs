use rand::Rng;

fn generate_dice(how_many: u32) -> Vec<u32> {
    let mut res = Vec::<u32>::new();

    for _ in 0..how_many {
        let random_die = rand::thread_rng().gen_range(1, 7);
        res.push(random_die);
    }

    return res;
}

fn main() {
    println!("intro...");

    let dice = generate_dice(2);
    println!("Dice: {:?}", dice);
}
