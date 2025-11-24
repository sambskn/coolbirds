use bevy::{ecs::resource::Resource, mesh::Mesh};
use csgrs::{mesh::plane::Plane, traits::CSG};
type CSGMesh = csgrs::mesh::Mesh<()>;
use bevy::log::info;

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

pub fn get_input_type_string(input_type: &BirdGenInputTypes) -> &str {
    match input_type {
        BirdGenInputTypes::BeakLength => "Beak Length",
        BirdGenInputTypes::BeakSize => "Beak Size",
        BirdGenInputTypes::BeakWidth => "Beak Width",
        BirdGenInputTypes::BeakRoundness => "Beak Roundness",
        BirdGenInputTypes::HeadSize => "Head Size",
        BirdGenInputTypes::HeadToBelly => "Head to Belly",
        BirdGenInputTypes::EyeSize => "Eye Size",
        BirdGenInputTypes::HeadLateralOffset => "Head Lateral Offset",
        BirdGenInputTypes::HeadLevel => "Head Level",
        BirdGenInputTypes::HeadYaw => "Head Yaw",
        BirdGenInputTypes::HeadPitch => "Head Pitch",
        BirdGenInputTypes::BellyLength => "Belly Length",
        BirdGenInputTypes::BellySize => "Belly Size",
        BirdGenInputTypes::BellyFat => "Belly Fat",
        BirdGenInputTypes::BellyToBottom => "Belly to Bottom",
        BirdGenInputTypes::BottomSize => "Bottom Size",
        BirdGenInputTypes::TailLength => "Tail Length",
        BirdGenInputTypes::TailWidth => "Tail Width",
        BirdGenInputTypes::TailYaw => "Tail Yaw",
        BirdGenInputTypes::TailPitch => "Tail Pitch",
        BirdGenInputTypes::TailRoundness => "Tail Roundness",
        BirdGenInputTypes::BaseFlat => "Base Flat",
    }
}

pub fn get_input_value_for_type(
    input_type: &BirdGenInputTypes,
    input_values: &BirdGenInputs,
) -> f32 {
    match input_type {
        BirdGenInputTypes::BeakLength => input_values.beak_length,
        BirdGenInputTypes::BeakSize => input_values.beak_size,
        BirdGenInputTypes::BeakWidth => input_values.beak_width,
        BirdGenInputTypes::BeakRoundness => input_values.beak_roundness,
        BirdGenInputTypes::HeadSize => input_values.head_size,
        BirdGenInputTypes::HeadToBelly => input_values.head_to_belly,
        BirdGenInputTypes::EyeSize => input_values.eye_size,
        BirdGenInputTypes::HeadLateralOffset => input_values.head_lateral_offset,
        BirdGenInputTypes::HeadLevel => input_values.head_level,
        BirdGenInputTypes::HeadYaw => input_values.head_yaw,
        BirdGenInputTypes::HeadPitch => input_values.head_pitch,
        BirdGenInputTypes::BellyLength => input_values.belly_length,
        BirdGenInputTypes::BellySize => input_values.belly_size,
        BirdGenInputTypes::BellyFat => input_values.belly_fat,
        BirdGenInputTypes::BellyToBottom => input_values.belly_to_bottom,
        BirdGenInputTypes::BottomSize => input_values.bottom_size,
        BirdGenInputTypes::TailLength => input_values.tail_length,
        BirdGenInputTypes::TailWidth => input_values.tail_width,
        BirdGenInputTypes::TailYaw => input_values.tail_yaw,
        BirdGenInputTypes::TailPitch => input_values.tail_pitch,
        BirdGenInputTypes::TailRoundness => input_values.tail_roundness,
        BirdGenInputTypes::BaseFlat => input_values.base_flat,
    }
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

// Bumping to 40 made my computer sad :(
// There is porbably a benefit to tuning the segement/stack count per geometry
const RESOLUTION_PSUEDO_UNIT: usize = 16;

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
    info!("Skull done");
    // beak
    info!("Making the beak");
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
    info!("Beak skelton done");
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
            info!("Making eye");
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
            info!("Put eye on head");
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
    info!("Body step 1, neck and chest");
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
    info!("Body step 2, bottom");
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
    info!("Body step 3, tail");
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
    info!("Body done");

    if input.base_flat > -100.0 {
        info!("Flattening base");
        let total_len =
            input.beak_length + input.head_to_belly + input.belly_to_bottom + input.tail_length;

        // Calculate the cut height (in OpenSCAD's z-axis, which becomes Bevy's y-axis after rotation)
        let cut_height = (input.belly_size * (-1.5 + input.base_flat / 200.0)) as f64;

        // Create a large cube to subtract from the bottom
        // Little hacky with my positioning but idc
        // Why doesn't it work in wasm? Makes me crazy!
        let cut_box = CSGMesh::cuboid(
            (total_len) as f64,
            (total_len) as f64,
            input.belly_size as f64,
            None,
        )
        .translate(-total_len as f64 / 4.0, -total_len as f64 / 2.0, cut_height);

        body = body.difference(&cut_box);
    }
    body
}

pub fn generate_bird_body_mesh(input: &BirdGenInputs) -> Mesh {
    let body = generate_bird_body_csg_mesh(input);
    // add the x axis rotation to account for y up world we're rocking with in bevy
    body.rotate(-90.0, 180.0, 0.0).to_bevy_mesh()
}

pub fn generate_full_bird_stl_string(input: &BirdGenInputs) -> String {
    let body = generate_bird_body_csg_mesh(input);
    let head_in_place = generate_bird_head_csg_mesh(input);

    info!("Generate STL strings, then combine");
    let body_stl_str = body.to_stl_ascii("bird");
    let head_stl_str = head_in_place.to_stl_ascii("just_head");
    // grab triangles from head and add to body
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

        result
    }
}

/* From https://www.thingiverse.com/thing:139945/files

// For Reference: The original Bird-o-Matic OpenSCAD code
// Generally csgrs seems to be a little mroe finnicky when doing unions of different solids
// E.g. the `scale` command for the eyes in the original code kinda borked the other eye when
//      implemented directly here (ended up using csgrs's mirror fn instead)
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
