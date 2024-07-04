pub mod sky;
pub mod terrain;
pub mod light;

pub struct World {
	pub sky: sky::Sky,
	// NOTE: the terrain is an entity at the end. pub terrain: terrain::ProceduralTerrain
}
