use godot::prelude::*;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use rand::Rng;
use itertools::Itertools;
use std::clone::Clone;

//In this file, the struct Point is used to refer to points on the graphs in    WorldGraph
//Vector3s refer to locations in the game world

pub const CHUNK_SIZE: i16 = 8;
pub const CHUNK_VERTICAL: i16 = 8;
pub const HORIZONTAL_SQUARE: i16 = CHUNK_SIZE.pow(2);
 
pub const SCATTER_AREA: i16 = 3 * CHUNK_SIZE;

//X, Y, Z
//Y IS VERTICAL
#[derive(Eq, Clone)]
pub struct Point(pub i16, pub i16, pub i16);

impl Ord for Point
{
    fn cmp(&self, other: &Self) -> Ordering 
    {
        if self.0 == other.0
        {
            if self.1 == other.1
            {
                return self.2.cmp(&other.2);
            }
            return self.1.cmp(&other.1);
        }
        return self.0.cmp(&other.0);
    }
}

impl PartialOrd for Point
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> 
    {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point 
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

//Will be used to store world gen information.
//The arrays level_graph and edges_graph are 1d arrays that will be address as 3d arrays
//through the function convert_point().
pub struct WorldGraph 
{
    seed_density: i16,

    level_graph: [i16; (HORIZONTAL_SQUARE * CHUNK_VERTICAL) as usize],
    edges_graph: [i16; (HORIZONTAL_SQUARE * CHUNK_VERTICAL) as usize],

    generated_chunks: BTreeSet<Point>,

    scattered_chunks: BTreeSet<Point>,
    seeds_list: BTreeSet<Point>,
}

pub fn create_world_graph(seed_density: i16) -> WorldGraph
{
    WorldGraph
    {
        seed_density,

        level_graph: [0i16; (HORIZONTAL_SQUARE * CHUNK_VERTICAL) as usize],
        edges_graph: [0i16; (HORIZONTAL_SQUARE * CHUNK_VERTICAL) as usize],

        generated_chunks: BTreeSet::new(),

        scattered_chunks: BTreeSet::new(),
        seeds_list: BTreeSet::new(),
    }

}

//Takes 3d Point in world space, mods each dimension down to the loaded size
//And converts to the proper location in the 1d array
fn convert_point(p: &Point) -> i16
{
    let n0 = p.0.rem_euclid (CHUNK_SIZE);
    let n1 = p.1.rem_euclid(CHUNK_VERTICAL);
    let n2 = p.2.rem_euclid (CHUNK_SIZE);

    n0 + (n2 * CHUNK_SIZE) + (n1 * HORIZONTAL_SQUARE)
}

pub fn get_level_point(graph:   &WorldGraph, p: &Point) -> i16
{
    graph.level_graph[convert_point(p) as usize]
}

pub fn get_edges_point(graph:   &WorldGraph, p: &Point) -> i16
{
    graph.edges_graph[convert_point(p) as usize]
}

pub fn set_level_point(graph: &mut  WorldGraph, p: &Point, val: i16)
{
    graph.level_graph[convert_point(p) as usize] = val;
}

pub fn set_edges_point(graph: &mut  WorldGraph, p: &Point, val: i16)
{
    graph.edges_graph[convert_point(p) as usize] = val;
}

fn calc_manhatten_dist(p1: &Point, p2: &Point) -> i16
{
    let dist_x = p1.0 - p2.0;
    let dist_y = p1.1 - p2.1;
    let dist_z = p1.2 - p2.2;

    dist_x.abs() + dist_y.abs() + dist_z.abs()
}

//Checks to see if seeds have already been generated in the region, then generates if not
pub fn scatter_seeds(graph: &mut WorldGraph, location: &Point)
{
    //Round location to the nearest multiple of SCATTER_AREA
    let rounded_loc = Point(
        ((location.0 as f32 / SCATTER_AREA as f32).round() as i16) * SCATTER_AREA,
        ((location.1 as f32 / SCATTER_AREA as f32).round() as i16) * SCATTER_AREA,
        ((location.2 as f32 / SCATTER_AREA as f32).round() as i16) * SCATTER_AREA,
    );

    let x = rounded_loc.0;
    let y = rounded_loc.1;
    let z = rounded_loc.2;

    if graph.scattered_chunks.insert(rounded_loc)
    {
        
        let mut rng = rand::thread_rng();

        for _i in 0..graph.seed_density
        {
            let mut success = false;

            while !success
            {
                let p = Point(
                    rng.gen_range(x..x + SCATTER_AREA), 
                    rng.gen_range(y..y + SCATTER_AREA),
                    rng.gen_range(z..z + SCATTER_AREA));
                success = graph.seeds_list.insert(p);
            }
        }
    }
}

pub fn find_regions(graph: &mut WorldGraph, location: &Point)
{
    let mut shortest_dist: i16;
    let mut shortest_index: usize;
    for i in location.0..location.0 + CHUNK_SIZE
    {
        for j in location.1..location.1 + CHUNK_VERTICAL
        {
            for k in location.2..location.2 + CHUNK_SIZE
            {
                shortest_dist = (2 * CHUNK_SIZE + CHUNK_VERTICAL) as i16;
                shortest_index = 0;

                let p1 = Point(i, j, k);

                for (x, s) in graph.seeds_list.iter().enumerate()
                {

                    let dist = calc_manhatten_dist(&p1, &s);

                    if dist < shortest_dist
                    {
                        shortest_dist = dist;
                        shortest_index = x;

                    }
                }
                set_level_point(graph, &p1, shortest_index as i16);
            }
        }
    }
}

pub fn convolutional_trace(graph: &mut  WorldGraph, location: &Point)
{
    let mut neighbors: [i16; 6] = [0, 0, 0, 0, 0, 0];

    for i in location.0..location.0 + CHUNK_SIZE
    {
        for j in location.1..location.1 + CHUNK_VERTICAL
        {
            for k in location.2..location.2 + CHUNK_SIZE
            {
                neighbors[0] = get_level_point(graph, &Point(i - 1, j, k));
                neighbors[1] = get_level_point(graph, &Point(i + 1, j, k));
                neighbors[2] = get_level_point(graph, &Point(i, j - 1, k));
                neighbors[3] = get_level_point(graph, &Point(i, j + 1, k));
                neighbors[4] = get_level_point(graph, &Point(i, j, k - 1));
                neighbors[5] = get_level_point(graph, &Point(i, j, k + 1));

                set_edges_point(graph, &Point(i, j, k), neighbors.into_iter().unique().count() as i16);
            }
        }
    }
}

//Calls find_regions() and convolutional_trace with a rounded location to ensure there is no duplicate generation
//returns true if new a chunk was generated, false if the chunk was already generated
pub fn generate_and_trace(graph: &mut  WorldGraph, location: &Point) -> bool
{
    let check_loc = Point(
        ((location.0 as f32 / CHUNK_SIZE as f32).round() as i16) * CHUNK_SIZE,
        ((location.1 as f32 / CHUNK_SIZE as f32).round() as i16) * CHUNK_SIZE,
        ((location.2 as f32 / CHUNK_SIZE as f32).round() as i16) * CHUNK_SIZE,
    );

    let rounded_loc = check_loc.clone();

    if graph.generated_chunks.insert(check_loc)
    {
        find_regions(graph, &rounded_loc);
        convolutional_trace(graph, &rounded_loc);
        return true;
    }
    return false;
}