// FOR BASE 1..N for N Bases
// Basically a scene with multiple entities.
// Things like units come in multiple characters. Their config will be stored in entity/groups

use crate::entity::worldobject::building::Building;

pub struct Base {
    // must be unique and have a radius of max(rad, 5/"control radius")
    map_location: (f32, f32),
    control_radius: f32,

    // SCOUTING BASE DATA
    buildings: Vec<Building>,
}

impl Base {
    pub fn new() -> Self {
        Self {
            map_location: (0.0, 0.0),
            control_radius: 5.0,
            buildings: vec![Building::new()],
        }
    }

    // scout this base, return base data
    pub fn scout(&self) {}
}
