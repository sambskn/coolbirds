use birdgen::BirdGenInputs;
use warp::Filter;

mod birdcache;

#[tokio::main]
async fn main() {
    let bird = warp::path!("bird" / String).then(async |bird_seed: String| {
        println!("bird req: {}", bird_seed);
        // Check if bird exists in cache
        let cache_check = birdcache::get_bird_from_cache(bird_seed.as_str()).await;
        match cache_check {
            Ok(bird_gltf_string) => bird_gltf_string,
            Err(bird_cache_error) => {
                println!("{:?}, generating bird...", bird_cache_error);
                // generate bird
                let mut bird_inputs = BirdGenInputs::default();
                bird_inputs
                    .update_from_seed_string(bird_seed.clone())
                    .unwrap();
                let gltf_str = bird_inputs.get_gltf().unwrap();
                // update cache
                let cache_add =
                    birdcache::add_bird_to_cache(gltf_str.as_str(), bird_seed.as_str()).await;
                match cache_add {
                    Ok(()) => {
                        println!("bird successfully added to cache")
                    }
                    Err(err) => {
                        println!("bird NOT added to cache, some err :( {:?}", err)
                    }
                };
                // return bird
                gltf_str
            }
        }
    });

    warp::serve(bird).run(([127, 0, 0, 1], 3030)).await;
}
