// The 'controller' in model-view-control framework
// SHOULD HAVE ACCESS TO THE CURRENT <MAP>/<LEVEL> when 'in game', i.e. completely loaded
// UPDATE THE VIEW WHEN SOMETHING CHANGES. Wait actually we can have 'caller prompted observer pattern" to update on demand at the next tick.
// * So the updates are queued for each entity. On the next tick, game controller checks entity update queue, goes to those entities and fetches the new state
// Then makes a render call using the new state

use crate::entity::Entity;

struct GameController {
    observed_entities: Vec<Entity>,
}

// attach game controller as a listener to an entity's state
fn listen_to(entity: &Entity) {}

// should be updated on every tick. No it doesnt take that much cpu/gpu
// just update it, even if nothing changes. If nothing changes then GPU wont need to completely re-render/load new assets and render a new scene
fn update_view() {}