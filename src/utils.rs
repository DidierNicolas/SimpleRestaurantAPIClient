use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::{ItemPair};

pub fn item_randomizer() -> Vec<ItemPair> {
    let mut items = Vec::<ItemPair>::new();
    let mut rng = rand::thread_rng();
    let nb_items = rand::thread_rng().gen_range(1..10);

    for _i in 0..(nb_items - 1) {
        items.push(ItemPair {
            name: random_string(),
            cook_time: rng.gen_range(5..15),
        });
    }
    items
}

pub fn random_string() -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
}

pub fn random_numb(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..max);
}