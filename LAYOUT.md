A terraformers-3d project should be basically a single cargo bin project representing an executable game
Either use it like a library or a viewer + library.
I like the second idea. An unreal proj will use visual studio organisation/CMAKE + custom c++ code organisation

So a demens project org could look:
backend/
    entities/
        code.ts
    shaders/
        code.ts
assets/
    textures/
    models/

where .ts is (terraform) terraformer script that just happens to look like typescript

A library only project looks just like a normal cargo project
you have access to the viewer but it wont have 100% of the GUI features
but you do have access to low level rust code

src/
    main.rs // main() fn to startup everything
    game.rs // game loops and etc. Start up your main game
    init.rs // initialise your game state from .config and save files, etc

    levels/
        map.rs // recommended for a strategy game or an rpg with a single big map
        mod.rs // level0 -> recommended as your title screen. Dont be like cities skylines and have your own launcher, pls!
               // level1..n -> recommended as your actual in game levels. For something like civ6, would be an actual game with a live map and players
                        // for minecraft, would be an in world game, survival, creative, etc.
                        // for battlefield, would be a live campaign map with enemies at certain positions, or a multiplayer map with set spawn points and entity spawn points
                        // for backyard monsters, would be your actual player base map, an attack map, defense map, the actual map of everything itself. Things like options, etc should be popups akin to imgui popups, not levels

    entities/
        mod.rs // your player character if doing a story mode thing or fps
                // if a strategy game, your characters and entities
        worldobject/
            mod.rs
        character/
            mod.rs

The code for graphics will be right next to code for movement. Well quite close at least. They should be somewhat separated but everything is in src/
Model-view-control is the key idea of a terraformer app. You should make changes to the backend state, which is then detected by the controller observer which then updates the frontend state. At every tick, dt.
