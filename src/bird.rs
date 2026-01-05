use bevy::{ecs::resource::Resource, mesh::Mesh};
use csgrs::{mesh::plane::Plane, traits::CSG};
use rand::seq::IndexedRandom;
type CSGMesh = csgrs::mesh::Mesh<()>;

const GOOD_BIRDS: &'static [&'static str] = &[
    "m.22.67.4.190.h.26.24.17.-7.26.33.36.b.43.21.55.13.35.t.36.15.-15.15.82.c.92",
    "m.44.67.16.131.h.20.1.3.10.26.30.37.b.57.21.55.13.35.t.31.15.-15.-6.82.c.94",
    "m.14.57.15.10.h.22.32.7.6.32.27.-22.b.60.40.100.40.25.t.50.47.14.72.197.c.100",
    "m.27.94.14.124.h.18.32.13.9.23.12.9.b.36.31.57.20.47.t.78.4.40.12.91.c.67",
    "m.15.79.5.120.h.32.-17.7.0.15.10.9.b.19.23.90.22.25.t.44.1.-5.40.65.c.-25",
    "m.2.70.16.131.h.6.1.3.10.36.30.-76.b.57.21.50.13.8.t.31.15.-15.-6.46.c.-20",
    "m.18.87.5.23.h.21.22.5.12.49.14.-24.b.7.58.121.5.17.t.72.23.-8.11.153.c.84",
    "m.1.86.3.145.h.29.16.14.8.15.-44.-43.b.53.48.149.6.37.t.28.6.27.-19.199.c.-30",
    "m.19.86.1.33.h.13.17.17.-4.1.6.34.b.27.26.103.17.45.t.30.19.39.33.102.c.-33",
    "m.23.88.3.118.h.28.25.18.5.67.-36.1.b.42.3.51.17.7.t.37.44.-18.-29.184.c.-81",
    "m.49.35.0.10.h.19.7.1.4.21.-15.-9.b.24.58.76.8.36.t.16.3.30.84.77.c.43"
];

// Inputs/descriptions copied from original Bird-o-matic .SCAD script (see referenced script at bottom of file)
// [Ed. note: Made em all f32's for now]
#[derive(Resource, Clone, Copy)]
pub struct BirdGenInputs {
    // Length of the beak
    pub beak_length: f32, // [0:50]
    // Ratio relative to the head size
    pub beak_size: f32, // [20:100]
    // Width of the beak tip (0 is pointy)
    pub beak_width: f32, // [0:25]
    // Shape of the beak tip (lowest is flat)
    pub beak_roundness: f32, // [10:200]

    // Head diameter
    pub head_size: f32, // [10:40]
    // Horizontal distance from head to main body
    pub head_to_belly: f32, // [-20:50]
    // Size of the eyes
    pub eye_size: f32, // [0:20]
    // Head lateral offset
    pub head_lateral_offset: f32, // [-15:15]
    // Head vertical height
    pub head_level: f32, // [0:80]
    // Head horizontal rotation
    pub head_yaw: f32, // [-45:45]
    // Head vertical rotation (positive is upwards)
    pub head_pitch: f32, // [-80:45]

    // How long is the front body
    pub belly_length: f32, // [10:100]
    // Belly section size
    pub belly_size: f32, // [20:60]
    // Additional fatness ratio
    pub belly_fat: f32, // [50:150]

    // Distance from main body center to bottom center
    pub belly_to_bottom: f32, // [1:50]
    // Bottom diameter
    pub bottom_size: f32, // [5:50]

    // Tail length
    pub tail_length: f32, //[0:100]
    // How large is the tail
    pub tail_width: f32, // [1:50]
    // Tail horizontal rotation
    pub tail_yaw: f32, // [-45:45]
    // Tail vertical angle (positive is upwards)
    pub tail_pitch: f32, // [-45:90]
    // How round is the tail (lowest is flat)
    pub tail_roundness: f32, // [10:200]

    // How to cut the base of the object (-1 to disable, then use your own slicer options)
    pub base_flat: f32, // [-100:100]
}

#[derive(Resource, Clone, Copy)]
pub struct RecentBirds {
    pub left: BirdGenInputs,
    pub right: BirdGenInputs,
}

pub enum BirdGenInputTypes {
    BeakLength,
    BeakSize,
    BeakWidth,
    BeakRoundness,
    HeadSize,
    HeadToBelly,
    EyeSize,
    HeadLateralOffset,
    HeadLevel,
    HeadYaw,
    HeadPitch,
    BellyLength,
    BellySize,
    BellyFat,
    BellyToBottom,
    BottomSize,
    TailLength,
    TailWidth,
    TailYaw,
    TailPitch,
    TailRoundness,
    BaseFlat,
}

impl Default for BirdGenInputs {
    fn default() -> Self {
        BirdGenInputs {
            beak_length: 15.0,
            beak_size: 80.0,
            beak_width: 5.0,
            beak_roundness: 10.0,
            head_size: 22.0,
            head_to_belly: 32.0,
            eye_size: 7.0,
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
            base_flat: 100.0,
        }
    }
}

impl BirdGenInputs {
    pub fn get_input_value_for_type(&self, input_type: &BirdGenInputTypes) -> f32 {
        match input_type {
            BirdGenInputTypes::BeakLength => self.beak_length,
            BirdGenInputTypes::BeakSize => self.beak_size,
            BirdGenInputTypes::BeakWidth => self.beak_width,
            BirdGenInputTypes::BeakRoundness => self.beak_roundness,
            BirdGenInputTypes::HeadSize => self.head_size,
            BirdGenInputTypes::HeadToBelly => self.head_to_belly,
            BirdGenInputTypes::EyeSize => self.eye_size,
            BirdGenInputTypes::HeadLateralOffset => self.head_lateral_offset,
            BirdGenInputTypes::HeadLevel => self.head_level,
            BirdGenInputTypes::HeadYaw => self.head_yaw,
            BirdGenInputTypes::HeadPitch => self.head_pitch,
            BirdGenInputTypes::BellyLength => self.belly_length,
            BirdGenInputTypes::BellySize => self.belly_size,
            BirdGenInputTypes::BellyFat => self.belly_fat,
            BirdGenInputTypes::BellyToBottom => self.belly_to_bottom,
            BirdGenInputTypes::BottomSize => self.bottom_size,
            BirdGenInputTypes::TailLength => self.tail_length,
            BirdGenInputTypes::TailWidth => self.tail_width,
            BirdGenInputTypes::TailYaw => self.tail_yaw,
            BirdGenInputTypes::TailPitch => self.tail_pitch,
            BirdGenInputTypes::TailRoundness => self.tail_roundness,
            BirdGenInputTypes::BaseFlat => self.base_flat,
        }
    }

    pub fn set_input_value_for_type(&mut self, input_type: &BirdGenInputTypes, value: f32) {
        match input_type {
            BirdGenInputTypes::BeakLength => self.beak_length = value,
            BirdGenInputTypes::BeakSize => self.beak_size = value,
            BirdGenInputTypes::BeakWidth => self.beak_width = value,
            BirdGenInputTypes::BeakRoundness => self.beak_roundness = value,
            BirdGenInputTypes::HeadSize => self.head_size = value,
            BirdGenInputTypes::HeadToBelly => self.head_to_belly = value,
            BirdGenInputTypes::EyeSize => self.eye_size = value,
            BirdGenInputTypes::HeadLateralOffset => self.head_lateral_offset = value,
            BirdGenInputTypes::HeadLevel => self.head_level = value,
            BirdGenInputTypes::HeadYaw => self.head_yaw = value,
            BirdGenInputTypes::HeadPitch => self.head_pitch = value,
            BirdGenInputTypes::BellyLength => self.belly_length = value,
            BirdGenInputTypes::BellySize => self.belly_size = value,
            BirdGenInputTypes::BellyFat => self.belly_fat = value,
            BirdGenInputTypes::BellyToBottom => self.belly_to_bottom = value,
            BirdGenInputTypes::BottomSize => self.bottom_size = value,
            BirdGenInputTypes::TailLength => self.tail_length = value,
            BirdGenInputTypes::TailWidth => self.tail_width = value,
            BirdGenInputTypes::TailYaw => self.tail_yaw = value,
            BirdGenInputTypes::TailPitch => self.tail_pitch = value,
            BirdGenInputTypes::TailRoundness => self.tail_roundness = value,
            BirdGenInputTypes::BaseFlat => self.base_flat = value,
        };
    }

    pub fn randomize_values(&mut self) {
        use rand::Rng;
        let mut rng = rand::rng();

        self.beak_length = rng.random_range(0.0..=50.0);
        self.beak_size = rng.random_range(20.0..=100.0);
        self.beak_width = rng.random_range(0.0..=25.0);
        self.beak_roundness = rng.random_range(10.0..=200.0);
        self.head_size = rng.random_range(10.0..=40.0);
        self.head_to_belly = rng.random_range(-20.0..=50.0);
        self.eye_size = rng.random_range(0.0..=20.0);
        self.head_lateral_offset = rng.random_range(-15.0..=15.0);
        self.head_level = rng.random_range(0.0..=80.0);
        self.head_yaw = rng.random_range(-45.0..=45.0);
        self.head_pitch = rng.random_range(-80.0..=45.0);
        self.belly_length = rng.random_range(10.0..=100.0);
        self.belly_size = rng.random_range(20.0..=60.0);
        self.belly_fat = rng.random_range(50.0..=150.0);
        self.belly_to_bottom = rng.random_range(1.0..=50.0);
        self.bottom_size = rng.random_range(5.0..=50.0);
        self.tail_length = rng.random_range(0.0..=100.0);
        self.tail_width = rng.random_range(1.0..=50.0);
        self.tail_yaw = rng.random_range(-45.0..=45.0);
        self.tail_pitch = rng.random_range(-45.0..=90.0);
        self.tail_roundness = rng.random_range(10.0..=200.0);
        self.base_flat = rng.random_range(-100.0..=100.0);
    }

    pub fn copy_from_other_bird(&mut self, other_bird: &BirdGenInputs) {
        self.beak_length = other_bird.beak_length;
        self.beak_size = other_bird.beak_size;
        self.beak_width = other_bird.beak_width;
        self.beak_roundness = other_bird.beak_roundness;
        self.head_size = other_bird.head_size;
        self.head_to_belly = other_bird.head_to_belly;
        self.eye_size = other_bird.eye_size;
        self.head_lateral_offset = other_bird.head_lateral_offset;
        self.head_level = other_bird.head_level;
        self.head_yaw = other_bird.head_yaw;
        self.head_pitch = other_bird.head_pitch;
        self.belly_length = other_bird.belly_length;
        self.belly_size = other_bird.belly_size;
        self.belly_fat = other_bird.belly_fat;
        self.belly_to_bottom = other_bird.belly_to_bottom;
        self.bottom_size = other_bird.bottom_size;
        self.tail_length = other_bird.tail_length;
        self.tail_width = other_bird.tail_width;
        self.tail_yaw = other_bird.tail_yaw;
        self.tail_pitch = other_bird.tail_pitch;
        self.tail_roundness = other_bird.tail_roundness;
        self.base_flat = other_bird.base_flat;
    }

    pub fn get_child_with(&self, mate: &BirdGenInputs) -> BirdGenInputs {
        use rand::Rng;
        let mut rng = rand::rng();
        let mut child = mate.clone();
        let all_bird_input_types = vec![
            BirdGenInputTypes::BeakLength,
            BirdGenInputTypes::BeakSize,
            BirdGenInputTypes::BeakWidth,
            BirdGenInputTypes::BeakRoundness,
            BirdGenInputTypes::HeadSize,
            BirdGenInputTypes::HeadToBelly,
            BirdGenInputTypes::EyeSize,
            BirdGenInputTypes::HeadLateralOffset,
            BirdGenInputTypes::HeadLevel,
            BirdGenInputTypes::HeadYaw,
            BirdGenInputTypes::HeadPitch,
            BirdGenInputTypes::BellyLength,
            BirdGenInputTypes::BellySize,
            BirdGenInputTypes::BellyFat,
            BirdGenInputTypes::BellyToBottom,
            BirdGenInputTypes::BottomSize,
            BirdGenInputTypes::TailLength,
            BirdGenInputTypes::TailWidth,
            BirdGenInputTypes::TailYaw,
            BirdGenInputTypes::TailPitch,
            BirdGenInputTypes::TailRoundness,
            BirdGenInputTypes::BaseFlat,
        ];
        // roll the dice for each 'trait'
        for bird_input_type in all_bird_input_types {
            // chance to prefer either parent's
            // some variance to try an represent more dominant traits? idk lol
            let parent_favor_per_trait = rng.random_range(0.4..=0.6);
            let prefer_mate_trait = rng.random_bool(parent_favor_per_trait);
            if !prefer_mate_trait {
                child.set_input_value_for_type(
                    &bird_input_type,
                    self.get_input_value_for_type(&bird_input_type),
                );
            }

            // chance to mutate
            let should_mutate = rng.random_bool(0.05);
            if should_mutate {
                let unmutated = child.get_input_value_for_type(&bird_input_type);
                let mutation_amount = rng.random_range(0.05..0.95); // only shrinks vals rn
                child.set_input_value_for_type(&bird_input_type, unmutated * mutation_amount);
            }
        }
        // return child
        child
    }

    pub fn get_bird_seed_string(&self) -> String {
        let mouth_str = format!(
            "m.{}.{}.{}.{}",
            self.beak_length as i32,
            self.beak_size as i32,
            self.beak_width as i32,
            self.beak_roundness as i32,
        );
        let head_str = format!(
            "h.{}.{}.{}.{}.{}.{}.{}",
            self.head_size as i32,
            self.head_to_belly as i32,
            self.eye_size as i32,
            self.head_lateral_offset as i32,
            self.head_level as i32,
            self.head_yaw as i32,
            self.head_pitch as i32,
        );
        let belly_str = format!(
            "b.{}.{}.{}.{}.{}",
            self.belly_length as i32,
            self.belly_size as i32,
            self.belly_fat as i32,
            self.belly_to_bottom as i32,
            self.bottom_size as i32,
        );
        let tail_str = format!(
            "t.{}.{}.{}.{}.{}",
            self.tail_length as i32,
            self.tail_width as i32,
            self.tail_yaw as i32,
            self.tail_pitch as i32,
            self.tail_roundness as i32,
        );
        let cutoff_str = format!("c.{}", self.base_flat as i32,);
        format!("{mouth_str}.{head_str}.{belly_str}.{tail_str}.{cutoff_str}")
    }

    pub fn update_from_seed_string(&mut self, seed: String) -> Result<(), String> {
        let parts: Vec<&str> = seed.split('.').collect();

        // Find section indices by looking for the prefixes
        let mut section_indices = Vec::new();
        for (i, part) in parts.iter().enumerate() {
            if matches!(*part, "m" | "h" | "b" | "t" | "c") {
                section_indices.push(i);
            }
        }

        // Add end index for easier slicing
        section_indices.push(parts.len());

        // Helper to parse section
        let parse_section = |start: usize, end: usize| -> Result<Vec<f32>, String> {
            parts[start + 1..end]
                .iter()
                .map(|s| {
                    s.parse::<f32>()
                        .map_err(|_| format!("Failed to parse: {}", s))
                })
                .collect()
        };

        // Parse each section
        for i in 0..section_indices.len() - 1 {
            let section_start = section_indices[i];
            let section_end = section_indices[i + 1];
            let prefix = parts[section_start];

            let values = parse_section(section_start, section_end)?;

            match prefix {
                "m" => {
                    if values.len() != 4 {
                        return Err(format!("Expected 4 mouth values, got {}", values.len()));
                    }
                    self.beak_length = values[0];
                    self.beak_size = values[1];
                    self.beak_width = values[2];
                    self.beak_roundness = values[3];
                }
                "h" => {
                    if values.len() != 7 {
                        return Err(format!("Expected 7 head values, got {}", values.len()));
                    }
                    self.head_size = values[0];
                    self.head_to_belly = values[1];
                    self.eye_size = values[2];
                    self.head_lateral_offset = values[3];
                    self.head_level = values[4];
                    self.head_yaw = values[5];
                    self.head_pitch = values[6];
                }
                "b" => {
                    if values.len() != 5 {
                        return Err(format!("Expected 5 belly values, got {}", values.len()));
                    }
                    self.belly_length = values[0];
                    self.belly_size = values[1];
                    self.belly_fat = values[2];
                    self.belly_to_bottom = values[3];
                    self.bottom_size = values[4];
                }
                "t" => {
                    if values.len() != 5 {
                        return Err(format!("Expected 5 tail values, got {}", values.len()));
                    }
                    self.tail_length = values[0];
                    self.tail_width = values[1];
                    self.tail_yaw = values[2];
                    self.tail_pitch = values[3];
                    self.tail_roundness = values[4];
                }
                "c" => {
                    if values.len() != 1 {
                        return Err(format!("Expected 1 cutoff value, got {}", values.len()));
                    }
                    self.base_flat = values[0];
                }
                _ => return Err(format!("Unknown section prefix: {}", prefix)),
            }
        }

        Ok(())
    }

    pub fn get_stl(&self) -> Result<Vec<u8>, std::io::Error> {
        // rotate STL's - idk man but when I uploaded to shapeways it thought the bird was on it's side, switching that up here
        let head_csg_mesh = generate_bird_head_csg_mesh(self).rotate(-90., 0., 0.);
        let body_csg_mesh = generate_bird_body_csg_mesh(self).rotate(-90., 0., 0.);
        let body_stl_str = body_csg_mesh
            .to_stl_ascii(format!("coolbird-{}", self.get_bird_seed_string()).as_str());
        let head_stl_str = head_csg_mesh.to_stl_ascii("head");
        // grab triangles from head and add to body, manually editing the string of the STL
        // (does feel a bit hacky - but it does maintain head and body triangles better)
        // much bigger than the binary format stl tho
        {
            let mut result = body_stl_str.clone();

            // Remove the "endsolid" line from body
            if let Some(pos) = result.rfind("endsolid") {
                result.truncate(pos);
            }

            // Extract facets from head (between "solid" line and "endsolid" line)
            let facets_start = head_stl_str.find("facet").unwrap_or(head_stl_str.len());
            let facets_end = head_stl_str.rfind("endsolid").unwrap_or(head_stl_str.len());
            let head_facets = &head_stl_str[facets_start..facets_end];

            // Combine: body (without endsolid) + head facets + endsolid
            result.push_str(head_facets);
            result.push_str("endsolid bird\n");

            Ok(result.as_bytes().to_vec())
        }
    }
    pub fn get_a_good_bird() -> Self {
        let mut rng = rand::rng();
        // select one of our strings of good birds
        let good_bird_str = *(GOOD_BIRDS.choose(&mut rng).unwrap());
        let mut output = BirdGenInputs::default();
        output
            .update_from_seed_string(good_bird_str.to_string())
            .unwrap();
        output
    }
}

// Bumping to 40 made my computer sad :(
// There is probably a benefit to tuning the segment/stack count per geometry
const RESOLUTION_PSUEDO_UNIT: usize = 20;

const SPHERE_SEGMENTS: usize = RESOLUTION_PSUEDO_UNIT;
const SPHERE_STACKS: usize = RESOLUTION_PSUEDO_UNIT * 2;

const NONZERO_THICKNESS: f64 = 0.1; // used in place of 0 when we want parts of the bird to approach an edge

// Currently making separate head and body meshes,
// Can't get a nice result when doing a union between the head and body
// (something in the csgrs Mesh union logic I think might be too aggressive at deleting triangles? -- armchair dev view lol)
// For now tho we'll just spawn two different meshes in Bevy, even though that would make printing it in 3d a bit harder.
// We'll see!

fn generate_bird_head_csg_mesh(input: &BirdGenInputs) -> CSGMesh {
    // skull base for head
    let skull: CSGMesh = CSGMesh::sphere(
        input.head_size as f64 / 2.0,
        2 * SPHERE_SEGMENTS,
        SPHERE_STACKS,
        None,
    );
    // beak
    let mut beak_skeleton: CSGMesh = CSGMesh::cylinder(
        if input.beak_width > 0.0 {
            input.beak_width as f64
        } else {
            NONZERO_THICKNESS
        },
        NONZERO_THICKNESS,
        SPHERE_SEGMENTS / 4, // way less resolution since we're conna covnex hull it
        None,
    )
    .scale(input.beak_roundness as f64 / 100.0, 1.0, 1.0)
    .translate(
        (-input.beak_length - input.head_size / 2.0) as f64,
        0.0,
        0.0,
    )
    .rotate(0.0, 15.0, 0.0)
    .union(&skull.clone());
    beak_skeleton.renormalize();
    let mut beak = beak_skeleton.convex_hull().scale(
        1.0,
        input.beak_size as f64 / 100.0,
        input.beak_size as f64 / 100.0,
    );
    beak.renormalize();
    // guess what, head is the beak now
    let mut head = beak;

    // eyes
    if input.eye_size > 0.0 {
        for y in [-1.0, 1.0] {
            let eye: CSGMesh = CSGMesh::sphere(
                input.eye_size as f64 / 2.0,
                // half resolution sphere compared to skull
                SPHERE_SEGMENTS / 2 + 2,
                SPHERE_STACKS / 2 + 2,
                None,
            )
            .scale(1.0, 1.0, 0.5)
            .translate(
                0.0,
                0.0,
                (input.head_size / 2.0 - input.eye_size / 8.0) as f64,
            )
            .rotate(50.0, -40.0, 0.0);
            // .scale(1.0, y, 1.0);
            if y == -1.0 {
                // flip one eye across y plane
                let plane_y = Plane::from_normal([0.0, 1.0, 0.0].into(), 0.0);
                head = head.union(&eye.mirror(plane_y));
            } else {
                head = head.union(&eye);
            }
            // important to do after unions to make sure the mesh looks nice
            // (i think lol)
            head.renormalize();
        }
    }

    let mut head_in_place = head
        .rotate(0.0, input.head_pitch as f64, input.head_yaw as f64)
        .translate(
            0.0,
            input.head_lateral_offset as f64,
            input.head_level as f64,
        )
        .scale(1.1, 1.1, 1.1);
    head_in_place.renormalize();
    head_in_place.subdivide_triangles(std::num::NonZero::<u32>::new(1).unwrap());
    head_in_place
}

pub fn generate_bird_head_mesh(input: &BirdGenInputs) -> Mesh {
    let head_in_place = generate_bird_head_csg_mesh(input);
    // add the x axis rotation to account for y up world we're rocking with in bevy
    head_in_place.rotate(-90.0, 180.0, 0.0).to_bevy_mesh()
}

pub fn generate_bird_body_csg_mesh(input: &BirdGenInputs) -> CSGMesh {
    let neck = CSGMesh::sphere(
        input.head_size as f64 / 2.0,
        SPHERE_SEGMENTS / 2 + 1,
        SPHERE_STACKS / 2 + 1,
        None,
    )
    .translate(
        0.0,
        input.head_lateral_offset as f64,
        input.head_level as f64,
    );
    let chest = CSGMesh::sphere(
        input.belly_size as f64 / 2.0,
        SPHERE_SEGMENTS + 2,
        SPHERE_STACKS + 2,
        None,
    )
    .scale(
        (input.belly_length / input.belly_size) as f64,
        input.belly_fat as f64 / 100.0,
        1.0,
    )
    .translate(input.head_to_belly as f64, 0.0, 0.0);
    let mut body = neck.union(&chest).convex_hull();
    let bottom = CSGMesh::sphere(
        input.bottom_size as f64 / 2.0,
        SPHERE_SEGMENTS + 1,
        SPHERE_STACKS + 1,
        None,
    )
    .translate(
        (input.head_to_belly + input.belly_to_bottom) as f64,
        0.0,
        0.0,
    );
    let body_plus_bottom = body.union(&bottom).convex_hull();
    body = body_plus_bottom;
    let tail = CSGMesh::cylinder(
        input.tail_width as f64,
        NONZERO_THICKNESS,
        SPHERE_SEGMENTS + 1,
        None,
    )
    .scale(input.tail_roundness as f64 / 100.0, 1.0, 1.0)
    .translate(input.tail_length as f64, 0.0, 0.0)
    .rotate(0.0, -input.tail_pitch as f64, input.tail_yaw as f64)
    .translate(
        (input.head_to_belly + input.belly_to_bottom) as f64,
        0.0,
        0.0,
    );
    let body_plus_tail = body.union(&tail).convex_hull();
    body = body_plus_tail;
    body.renormalize();

    if input.base_flat > -100.0 {
        let total_len =
            input.beak_length + input.head_to_belly + input.belly_to_bottom + input.tail_length;

        // Calculate the cut height (in OpenSCAD's z-axis, which becomes Bevy's y-axis after rotation)
        let cut_height = (input.belly_size * (-1.5 + input.base_flat / 200.0)) as f64;

        // Create a large cube to subtract from the bottom
        // Little hacky with my positioning but idc
        let cut_box = CSGMesh::cuboid(
            (total_len * 4.0) as f64,
            (total_len * 4.0) as f64,
            input.belly_size as f64 + (total_len * 4.0) as f64,
            None,
        )
        .translate(
            -total_len as f64 * 2.0,
            -total_len as f64 * 2.0,
            cut_height - total_len as f64 * 4.0,
        );

        body = body.difference(&cut_box);
    }
    body
}

pub fn generate_bird_body_mesh(input: &BirdGenInputs) -> Mesh {
    let body = generate_bird_body_csg_mesh(input);
    // add the x axis rotation to account for y up world we're rocking with in bevy
    body.rotate(-90.0, 180.0, 0.0).to_bevy_mesh()
}

/* From https://www.thingiverse.com/thing:139945/files

// For Reference: The original Bird-o-Matic OpenSCAD code
// Generally csgrs seems to be a little more finnicky when doing unions of different solids
// E.g. the `scale` command for the eyes in the original code kinda borked the other eye when
//      implemented directly here (ended up using csgrs's mirror fn instead)
// Big ty to the original author ('mooncactus' on thingiverse)!
// ~~~ORIGINAL CODE BELOW~~~

// Better use "fast" when tuning your bird, then "hi" to print it
precision="low"; // [low,med,hi]

// Length of the beak
beak_length= 15; // [0:50]
// Ratio relative to the head size
beak_size= 100; // [20:100]
// Width of the beak tip (0 is pointy)
beak_width= 0; // [0:25]
// Shape of the beak tip (lowest is flat)
beak_roundness= 10; // [10:200]

// Head diameter
head_size=22; // [10:40]
// Horizontal distance from head to main body
head_to_belly=32; // [-20:50]
// Size of the eyes
eye_size=0; // [0:20]
// Head lateral offset
head_lateral_offset=4; // [-15:15]
// Head vertical height
head_level=32; // [0:80]
// Head horizontal rotation
head_yaw=10; // [-45:45]
// Head vertical rotation (positive is upwards)
head_pitch=9; // [-80:45]

// How long is the front body
belly_length=60; // [10:100]
// Belly section size
belly_size=40; // [20:60]
// Additional fatness ratio
belly_fat=90; // [50:150]

// Distance from main body center to bottom center
belly_to_bottom=25; // [1:50]
// Bottom diameter
bottom_size=25; // [5:50]

// Tail length
tail_length= 50; //[0:100]
// How large is the tail
tail_width= 22; // [1:50]
// Tail horizontal rotation
tail_yaw=-5; // [-45:45]
// Tail vertical angle (positive is upwards)
tail_pitch=40; // [-45:90]
// How round is the tail (lowest is flat)
tail_roundness=80; // [10:200]

// How to cut the base of the object (-1 to disable, then use your own slicer options)
base_flat= 50; // [-100:100]

$fa= ( precision=="low" ? 10 : ( precision=="med" ? 5 : 3) );
$fs= ( precision=="low" ? 8 : ( precision=="med" ? 3 : 1.8) );
total_len= beak_length+head_to_belly+belly_to_bottom+tail_length;

module chained_hull()
{
    for(i=[0:$children-2])
        hull()
            for(j=[i,i+1])
                child(j);
}

module skull()
{
    sphere(r=head_size/2);
}

module head()
{
    skull();
    if(eye_size>1)
        for(y=[-1,+1])
            scale([1,y,1])
                rotate([50,-40,0])
                    translate([0,0,head_size/2-eye_size/8])
                        scale([1,1,0.5])
                            sphere(r=eye_size/2, $fs=1);

    scale([1, beak_size/100, beak_size/100])
        hull()
        {
            skull();
            rotate([0,15,0])
                translate([-beak_length-head_size/2,0,0])
                    scale([beak_roundness/100,1,1])
                        cylinder(r=beak_width?beak_width:0.1,h=0.1); // nose
        }
}

translate([0,0,bottom_size/2])
difference()
{
    translate([-head_to_belly,0,0])
    union()
    {
        translate([0,head_lateral_offset,head_level])
            rotate([0,head_pitch,head_yaw])
                head();

        chained_hull()
        {
            translate([0,head_lateral_offset,head_level])
                sphere(r=head_size/2);

            translate([head_to_belly,0,0])
                scale([belly_length/belly_size,belly_fat/100,1])
                    sphere(r=belly_size/2);

            translate([head_to_belly+belly_to_bottom,0,0])
                    sphere(r=bottom_size/2);

            if(tail_length && tail_width && tail_roundness)
            translate([head_to_belly+belly_to_bottom,0,0])
                rotate([0,-tail_pitch,tail_yaw])
                    translate([tail_length,0,0])
                        scale([tail_roundness/100,1,1])
                            cylinder(r=tail_width,h=0.1);
        }
    }
    if(base_flat!=-100)
        translate([-total_len*5,-total_len*5,belly_size*(-1.5 + base_flat/200) ])
            cube([total_len*10,total_len*10,belly_size]);
}

*/
