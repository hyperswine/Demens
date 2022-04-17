# Demens

A game based on backyard monsters made entirely in terraformers-3d/rust framework. Optimised for quantii.

## Terraformer3D Usage

- [ ] Game loop with menus and levels (menus are level-agnostic and global state changing and should be instant, no need to restart)
- [ ] Physics engine to calculate per-frame character movement based on external forces applied, collisions and speed. Uses graphics hardware to accelerate physics
- [ ] RT for rendering each scene rather than usual wgpu phong shading
- [ ] Model, mesh and texture loading wrappers
- [ ] Audio loading wrappers
- [ ] 3D map + top down 3D camera like DotA2
- [ ] Logic to bind GameModels to structs
- [ ] Logic to tell structs/objects what to do, move in the x,y,z coords
- [ ] AI pathing `path!` and choice `choose!` mechanics
- [ ] Scripting macros `script!` to move objects
