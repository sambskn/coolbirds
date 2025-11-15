use bevy::mesh::Mesh;
use csgrs::traits::CSG;
type CSGMesh = csgrs::mesh::Mesh<()>;
use bevy::log::info;

pub struct BirdGenInputs {
    // Length of the beak
    beak_length: f32, // [0:50]
    // Ratio relative to the head size
    beak_size: f32, // [20:100]
    // Width of the beak tip (0 is pointy)
    beak_width: f32, // [0:25]
    // Shape of the beak tip (lowest is flat)
    beak_roundness: f32, // [10:200]

    // Head diameter
    head_size: f32, // [10:40]
    // Horizontal distance from head to main body
    head_to_belly: f32, // [-20:50]
    // Size of the eyes
    eye_size: f32, // [0:20]
    // Head lateral offset
    head_lateral_offset: f32, // [-15:15]
    // Head vertical height
    head_level: f32, // [0:80]
    // Head horizontal rotation
    head_yaw: f32, // [-45:45]
    // Head vertical rotation (positive is upwards)
    head_pitch: f32, // [-80:45]

    // How long is the front body
    belly_length: f32, // [10:100]
    // Belly section size
    belly_size: f32, // [20:60]
    // Additional fatness ratio
    belly_fat: f32, // [50:150]

    // Distance from main body center to bottom center
    belly_to_bottom: f32, // [1:50]
    // Bottom diameter
    bottom_size: f32, // [5:50]

    // Tail length
    tail_length: f32, //[0:100]
    // How large is the tail
    tail_width: f32, // [1:50]
    // Tail horizontal rotation
    tail_yaw: f32, // [-45:45]
    // Tail vertical angle (positive is upwards)
    tail_pitch: f32, // [-45:90]
    // How round is the tail (lowest is flat)
    tail_roundness: f32, // [10:200]

    // How to cut the base of the object (-1 to disable, then use your own slicer options)
    base_flat: f32, // [-100:100]
}

impl Default for BirdGenInputs {
    fn default() -> Self {
        BirdGenInputs {
            beak_length: 15.0,
            beak_size: 100.0,
            beak_width: 0.0,
            beak_roundness: 10.0,
            head_size: 22.0,
            head_to_belly: 32.0,
            eye_size: 5.0,
            head_lateral_offset: 4.0,
            head_level: 32.0,
            head_yaw: 10.0,
            head_pitch: 9.0,
            belly_length: 60.0,
            belly_size: 40.0,
            belly_fat: 90.0,
            belly_to_bottom: 25.0,
            bottom_size: 25.0,
            tail_length: 50.0,
            tail_width: 22.0,
            tail_yaw: -5.0,
            tail_pitch: 40.0,
            tail_roundness: 80.0,
            base_flat: 50.0,
        }
    }
}

const RESOLUTION_PSUEDO_UNIT: usize = 20;

const SPHERE_SEGMENTS: usize = RESOLUTION_PSUEDO_UNIT;
const SPHERE_STACKS: usize = RESOLUTION_PSUEDO_UNIT * 2;

pub fn generate_bird_mesh(input: BirdGenInputs) -> Mesh {
    info!("Start that bird");
    let skull: CSGMesh = CSGMesh::sphere(
        input.head_size as f64 / 2.0,
        SPHERE_SEGMENTS,
        SPHERE_STACKS,
        None,
    );
    let mut head = skull;
    info!("Skull done");
    if input.eye_size > 0.0 {
        for y in [-1.0, 1.0] {
            info!("Making eye");
            let eye: CSGMesh = CSGMesh::sphere(
                input.eye_size as f64 / 2.0,
                // half resolution sphere compared to skull
                SPHERE_SEGMENTS / 2,
                SPHERE_STACKS / 2,
                None,
            )
            .scale(1.0, 1.0, 0.5)
            .translate(
                0.0,
                0.0,
                (input.head_size / 2.0 - input.eye_size / 8.0) as f64,
            )
            .rotate(50.0, -40.0, 0.0)
            .scale(1.0, y, 1.0);
            info!("Put eye on head");
            head = head.union(&eye);
        }
    }

    // TODO: Beak and body, yknow, the rest...

    info!("Make bevy mesh");
    // add the x axis rotation to account for y up world
    head.rotate(-90.0, 180.0, 0.0).to_bevy_mesh()
}
