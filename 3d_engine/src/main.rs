fn main() {
    println!("Hello, world!");
    // WHAT SHOULD BE DEFINED at the end.
    
    // INTERACTION structs -> vec
    // ACTION structs -> vec
    // CONTROLLER -> matching VIRTUALKEYCODE to ACTION/INTERACTION
    // create the NCP enum
    
    // Entity load
    // World load
    // Scene creation
    // Camera creation
    // window configuration
    // shader creation
    
    // 0. STATE - HASHMAP OF SCENES
    // 1. CREATE A SCENE
	// - create entities( set NCPs/Player)
	// - create cameras ( one for the Player Entity, others for the world)
	// - create interactions/actions
	// - create a controller( hashmap VIRTUALKEYCODE-interaction, movement)
	//	 the controller will move the player and the camera
	// - create world(sky -> stars, procedural terrain, light)
	// - Pipelines and rendering code
    //
	// - EVERYTHING THAT HAS MOVED SHOULD BE UPDATED
	// - I HAVE TO UPDATE:
	//		each mesh(vertex buffer, raw_vertices) everytime we move
	//      the camera(projection, rotation shit?) everytime we move
    

    engine::user::game();
}
