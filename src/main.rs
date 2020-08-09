fn generate_dice(how_many: u32) -> Vec<u32> {
    let mut res = Vec::<u32>::new();

    for i in 0..how_many {
        res.push(i);
    }

    return res;
}

fn main() {
    println!("intro...");

    let dice = generate_dice(2);
    println!("Dice: {:?}", dice);
}
