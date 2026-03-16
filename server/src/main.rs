use birdgen::BirdGenInputs;
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("bird" / String).map(|bird_seed| {
        let mut bird_inputs = BirdGenInputs::default();
        println!("make a bird: {:?}", bird_seed);
        bird_inputs.update_from_seed_string(bird_seed).unwrap();
        let gltf_str = bird_inputs.get_gltf().unwrap();
        println!("Done!");
        gltf_str
    });

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
