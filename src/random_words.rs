use rand::seq::IndexedRandom;

pub fn get_random_exaclamatory() -> String {
    let mut rng = rand::rng();
    let sources = vec![
        "wowza",
        "cowabunga",
        "slamtastic",
        "bada bing",
        "call your mom",
    ];
    let chosen_word = *sources.choose(&mut rng).unwrap();
    chosen_word.to_string()
}
