pub(crate) mod character;
pub(crate) mod groups;
pub(crate) mod worldobject;

// TODO: remove the stuff in character and use Engagabable/Combatant instead
pub trait Combatant {
    // all entities should only have a single attack. No multi attacks in backyard monsters, just basic attack
    fn attack(&self);
    fn defend(&self);
}

// Most things should be displaceable, except maybe intrinsic map decors
// user decors, buildings and characters all displaceable
pub trait Displaceable {
    fn move_to(&self, coords: (f32, f32));
}

pub struct Entity {}
