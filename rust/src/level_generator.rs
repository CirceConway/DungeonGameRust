use godot::prelude::*;
use godot::engine::MeshInstance3D;
use crate::world_graph::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
struct LevelGenerator
{
    tile_size: f32,
    graph: WorldGraph,

    test_tile: Gd<PackedScene>,

    spawn_location: Vector3,

    base: Base<Node3D>
}

#[godot_api]
impl INode3D for LevelGenerator
{
    fn init(base: Base<Node3D>) -> Self 
    {
        godot_print!("Hello, world!"); // Prints to the Godot console
        
        Self 
        {
            tile_size: 1.0,
            graph: create_world_graph(10),

            test_tile: PackedScene::new_gd(),

            spawn_location: Vector3::new(0.0, 0.0, 0.0),

            base,
        }
    }

    fn ready(&mut self)
    {
        self.test_tile = load("res://test_tile.tscn");
        self.gen_starting_area();
    }
    
    fn physics_process(&mut self, delta: f64) 
    {
        //self.spawn_location.x = self.spawn_location.x + 10.0 * delta as f32;
        //let bad_location = self.vec_to_point(self.spawn_location);
        //self.gen_region(&bad_location);
    }


}
#[godot_api]
impl LevelGenerator
{
    fn vec_to_point(&self, vec: Vector3) -> Point
    {
        Point((vec.x / self.tile_size) as i16, (vec.y / self.tile_size) as i16, (vec.z / self.tile_size) as i16)
    }

    fn spawn_tiles(&mut self, loc_point: &Point)
    {
        //godot_print!("Spawning Things"); // Prints to the Godot console
        //let mut test_tile = self.test_tile.instantiate_as::<MeshInstance3D>();

        //let loc_point = self.vec_to_point(self.base().get_position());

        for i in loc_point.0..(loc_point.0 + CHUNK_SIZE)
        {
            for j in loc_point.1..(loc_point.1 + CHUNK_VERTICAL)
            {
                for k in loc_point.2..(loc_point.2 + CHUNK_SIZE)
                {
                    //godot_print!("Point {}, {}, {}, val: {}", i, j, k, get_edges_point(&self.graph, &Point(i, j, k)));
                    if get_edges_point(&self.graph, &Point(i, j, k)) > 2
                    {
                        let mut test_tile = self.test_tile.instantiate_as::<MeshInstance3D>();
                        
                        test_tile.set_position(Vector3::new(i as f32 * self.tile_size, j as f32 * self.tile_size, k as f32 * self.tile_size));
                        self.base_mut().add_child(test_tile.clone().upcast());
                    }
                }
            }
        }
    }

    fn gen_starting_area(&mut self)
    {
        let spawn_size = 12;
        //godot_print!("Generating");
        //let loc_point = Point(0, 0, 0);

        for i in 0..spawn_size
        {
            for j in 0..spawn_size
            {
                for k in 0..spawn_size
                {
                    let loc_point = Point(i * CHUNK_SIZE, j * CHUNK_SIZE, k * CHUNK_SIZE);
                    self.gen_region(&loc_point);
                    godot_print!("Generated a Chunk");
                }
            }
        }

        
    }

    fn gen_region(&mut self, loc_point: &Point)
    {
        scatter_seeds(&mut self.graph, &loc_point);
        if generate_and_trace(&mut self.graph, &loc_point)
        {
            self.spawn_tiles(&loc_point);
        }
    }
}