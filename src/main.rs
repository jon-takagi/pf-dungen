#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use rand::Rng;
mod types;
mod renderer;
mod exporter;
use crate::types::{Room, DUNGEON_HEIGHT, DUNGEON_WIDTH};
// use crate::types::{Monster, Encounter, EMPTY_ENCOUNTER};
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
// This tool uses trademarks and/or copyrights owned by Paizo Inc., used under Paizo's Community Use Policy (paizo.com/communityuse). 
// We are expressly prohibited from charging you to use or access this content. 
// This tool is not published, endorsed, or specifically approved by Paizo. 
// For more information about Paizo Inc. and Paizo products, visit paizo.com.

const NUM_ROOMS:i32                     = 16;
const ATTEMPT_THRESHOLD:i32             = 50; // keep trying until i break
const CENTRAL_ROOM_WIDTH_THRESHOLD:u32  = 30;
const CENTRAL_ROOM_HEIGHT_THRESHOLD:u32 = 30;
fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let mut rooms:Vec<Room> = Vec::new();

    let mut rng = rand::thread_rng();
    let mut room_index = 0;
    let mut failed_attempts = 0;

    let ROOM_DIMENSIONS = [(40, 40), (50, 50), (30, 40), (40, 50), (50, 80), (30, 30), (20, 20), (20, 30)];

    // let test_room = Room{
    //     id: 0,
    //     height: 10,
    //     width: 20,
    //     top_left: (10, 20),
    //     purpose: "none".to_string(),
    //     encounter: None
    // };
    // println!("test room top left is ({}, {})", test_room.top_left.0, test_room.top_left.1);
    // println!("test room bottom right is ({}, {})", test_room.bottom_right().0, test_room.bottom_right().1);
    // println!("test room center is ({}, {})", test_room.center_point().0, test_room.center_point().1);
    // rooms.push(test_room);
    // room_index += 1;

    // let test_room_2 = Room
    // {
    //     id: 1,
    //     height: 10,
    //     width: 20,
    //     top_left: (20, 20),
    //     purpose: "none".to_string(),
    //     encounter: None
    // };

    // println!("test_room_2 top left is ({}, {})", test_room_2.top_left.0, test_room_2.top_left.1);
    // println!("test_room_2 bottom right is ({}, {})", test_room_2.bottom_right().0, test_room_2.bottom_right().1);
    // println!("test_room_2 center is ({}, {})", test_room_2.center_point().0, test_room_2.center_point().1);
    // rooms.push(test_room_2);
    
    while (rooms.len() as i32) < NUM_ROOMS
    {
        if ATTEMPT_THRESHOLD > 0 && failed_attempts == ATTEMPT_THRESHOLD
        {
            println!("attempt threshold reached");
            break;
        }
        let room_key = rng.gen_range(0..ROOM_DIMENSIONS.len());

        let h = ROOM_DIMENSIONS[room_key].0;
        let w = ROOM_DIMENSIONS[room_key].1;

        let x:u32 = rng.gen_range(0..DUNGEON_WIDTH);
        let y:u32 = rng.gen_range(0..DUNGEON_HEIGHT);

        // do not create rooms that extend past the edges of the dungeon.

        let new_room = Room {
            id: room_index,
            height: h,
            width: w,
            top_left: (x, y),
            purpose: None,
            encounter: None
        };

        if new_room.is_inside_dungeon()
        {
    
            // let mut overlaps:bool = false;
            // for room in rooms.iter()
            // {
            //     if overlaps 
            //     {
            //         break;
            //     }
            //     overlaps = overlaps || new_room.overlaps(room);
            // }            
            // if !overlaps
            // {
                rooms.push(new_room);
                failed_attempts = 0;
                room_index += 1;
            // }
        }
        failed_attempts += 1;
    }

    // let goblin1 = Monster{name: "goblin1".to_string(), level:-1, aon_link:"https://2e.aonprd.com/Monsters.aspx?ID=232".to_string()};
    // let goblin2 = Monster{name: "goblin2".to_string(), level:-1, aon_link:"https://2e.aonprd.com/Monsters.aspx?ID=232".to_string()};
    // let goblin3 = Monster{name: "goblin3".to_string(), level:-1, aon_link:"https://2e.aonprd.com/Monsters.aspx?ID=232".to_string()};
    // let goblin4 = Monster{name: "goblin4".to_string(), level:-1, aon_link:"https://2e.aonprd.com/Monsters.aspx?ID=232".to_string()};
    // let goblin_ambush = Encounter {level:1, xp_award:60, loot:vec!["13gp".to_string()], monsters:vec![goblin1, goblin2, goblin3, goblin4]};
    let central_points = find_central_rooms(&mut rooms);
    renderer::render(DUNGEON_WIDTH, DUNGEON_HEIGHT, &mut rooms, "dungen-preview.png".to_string(), &central_points);
    println!("generated {} rooms, beginning separation", rooms.len());
    // let central_points = find_central_rooms(&mut rooms);
    separate_rooms(DUNGEON_WIDTH, DUNGEON_HEIGHT, &mut rooms);
    let central_points = find_central_rooms(&mut rooms);

    exporter::export_floor(&mut rooms, &("dungen.json".to_string()));
    renderer::render(DUNGEON_WIDTH, DUNGEON_HEIGHT, &mut rooms, "dungen.png".to_string(), &central_points);
    println!("found {} central points", central_points.len());
    for point in &central_points
    {
        println!("center point at: ({}, {})", point.x, point.y);
    }
    let triangulation: delaunator::Triangulation = delaunator::triangulate(&central_points);
    // println!("{:?}", triangulation.triangles); // [0, 2, 1, 0, 3, 2]
    // for room in &rooms 
    // {
    //     let id = room.id;
    //     let h = room.height;
    //     let w = room.width;
    //     let top_left = room.top_left;
    //     println!("room {0} top_left at ({3}, {4}) with height {1} and width {2}", id, h, w, top_left.0, top_left.1);
    // }

    // let a_vector = vec!["hello, ", "world!"];
    // let concat_iter = a_vector.iter().fold("".to_string(), |acc, element| acc + element);
    // println!("{}", concat_iter);
    
    // let mut acc = "".to_string();
    // for element in &a_vector
    // {
    //     acc = acc + element;
    // }
    // println!("{}", acc);


    // let edges:Vec::<(delaunator::Point, delaunator::Point)> = (0..triangulation.triangles.len()).fold(Vec::new(), |acc, e| 
    // {
    //     if e > triangulation.halfedges[e] || triangulation.halfedges[e] == delaunator::EMPTY 
    //     {
    //         let start = &central_points[triangulation.triangles[e]];
    //         let end = &central_points[triangulation.triangles[delaunator::next_halfedge(e)]];
    //         // acc.push((start.clone(), end.clone()));
    //         println!("edge from ({}, {}) to ({}, {})", 
    //             start.x, 
    //             start.y, 
    //             end.x, 
    //             end.y);
    //         acc
    //     }
    // });

    let central_room_ids = find_central_room_ids(&mut rooms);
    let mut graph = Graph::<u32, f64, petgraph::Undirected>::new_undirected();
    let mut center_point_to_node_index_map = std::collections::HashMap::<(u32, u32), NodeIndex>::new();    
    for i in 0..central_room_ids.len()
    {
        let room = &rooms[central_room_ids[i] as usize];
        center_point_to_node_index_map.insert(room.center_point(), graph.add_node(central_room_ids[i]));        
    }
    for i in 0..triangulation.triangles.len()
    {
        if i > triangulation.halfedges[i] || triangulation.halfedges[i] == delaunator::EMPTY 
        {
            let start = &central_points[triangulation.triangles[i]];
            let end = &central_points[triangulation.triangles[delaunator::next_halfedge(i)]];
            println!("edge from ({}, {}) to ({}, {})", 
                start.x, 
                start.y, 
                end.x, 
                end.y);
            let square_dist: f64 = (end.x - start.x).powf(2.0) + (end.y - start.y).powf(2.0);
            let start = center_point_to_node_index_map[&(start.x as u32, start.y as u32)];
            let end = center_point_to_node_index_map[&(end.x as u32, end.y as u32)];
            graph.add_edge(start, end, square_dist);
        }
    }


    // for edge in &edges
    // {
    //     println!("edge from ({}, {}) to ({}, {})", edge.0.x, edge.0.y, edge.1.x, edge.1.y);
    // }

    // let edges = Vec::<(u32, u32)>::new();
    // for i in 0..triangulation.triangles.len() -1
    // {
    //     // edges.push(triangulation.triangles[i], triangulation.triangles[i+1]);
    // }


    // renderer::render(DUNGEON_WIDTH, DUNGEON_HEIGHT, &mut rooms, "dungeon.png".to_string(), &central_points);
    renderer::draw_triangulation(DUNGEON_WIDTH, DUNGEON_HEIGHT, &central_points, &triangulation)
}

fn separate_rooms(dungeon_width:u32, dungeon_height:u32,rooms: &mut Vec<Room>)
{
    loop
    {
        for id_1 in 0..rooms.len()
        {
            for id_2 in 0..rooms.len()
            {
                let first_room:&Room = &rooms[id_1];
                let second_room:&Room = &rooms[id_2]; 
                if id_1 == id_2 || !first_room.overlaps(&second_room)
                {
                    continue;
                }

                // println!("first_room.center ({}, {})", first_room.center_point().0 as f64, first_room.center_point().1);
                // println!("second room.center {} {} ", second_room.center_point().0 as f64, second_room.center_point().1);
                // return;
                let p1 = delaunator::Point {x: first_room.center_point().0 as f64, y: first_room.center_point().1 as f64};
                let p2 = delaunator::Point {x: second_room.center_point().0 as f64, y: second_room.center_point().1 as f64};
                let difference = (p2.x - p1.x, p2.y - p1.y);
                // println!("comparing id {} to id {}", id_1, id_2);
                // println!("p1: {}, {}", p1.x, p1.y);
                // println!("p2: {}, {}", p2.x, p2.y);
                // println!("difference: {}, {}", difference.0, difference.1);
                let length = ((difference.0.powf(2.0) + difference.1.powf(2.0)) as f64).sqrt();

                let x_velocity = (difference.0 as f64/ length).round() as i32;
                let y_velocity = (difference.1 as f64/ length).round() as i32;
                // println!("velocity is {} {}", x_velocity, y_velocity);

                drop(first_room);
                drop(second_room);
                let mutable_room:&mut Room = &mut rooms[id_2];
                let ret_val = mutable_room.separate_by((x_velocity, y_velocity), dungeon_width, dungeon_height);
                let mutable_room:&mut Room = &mut rooms[id_1];
                if !ret_val
                {
                    mutable_room.separate_by((-2 * x_velocity, -2 * y_velocity), dungeon_width, dungeon_height);
                }
                else
                {
                    mutable_room.separate_by((-1 * x_velocity, -1 * y_velocity), dungeon_width, dungeon_height);
                }
            }
        }
        if !any_rooms_overlap(rooms)
        {
            break;
        }
        // i += 1;
        // if i % 10000 == 0
        // {
        //     println!("on iteration {}", i);
        //     let filename: String = format!("dungeon{}.png", i);
        //     renderer::render(DUNGEON_WIDTH, DUNGEON_HEIGHT, rooms, filename, &a_vec);
        // }
    }
}


fn any_rooms_overlap(rooms:&Vec<Room>) -> bool
{
    let mut overlaps = false;
    for first_index in 0..rooms.len()
    {
        for second_index in 0..rooms.len()
        {
            if first_index != second_index
            {
                overlaps = overlaps ||
                    rooms.get(first_index).unwrap().overlaps(rooms.get(second_index).unwrap()) || 
                    rooms.get(first_index).expect("REASON").bottom_right().0 > DUNGEON_WIDTH ||
                    rooms.get(first_index).expect("REASON").bottom_right().1 > DUNGEON_HEIGHT;
            }
        }
    }
    return overlaps;
}

// todo - combine methods
fn find_central_rooms(rooms: &mut Vec<Room>) -> Vec<delaunator::Point>
{
    let mut ret_val = Vec::<delaunator::Point>::new();
    for room in rooms
    {
        if room.height > CENTRAL_ROOM_HEIGHT_THRESHOLD && room.width > CENTRAL_ROOM_WIDTH_THRESHOLD
        {
            let p: delaunator::Point = delaunator::Point {x: room.center_point().0 as f64, y: room.center_point().1 as f64};
            ret_val.push(p);
        }
    }
    return ret_val;
}

fn find_central_room_ids(rooms : &mut Vec<Room>) -> Vec<u32>
{
    let mut ret_val = Vec::<u32>::new();
    for room in rooms
    {
        if room.height > CENTRAL_ROOM_HEIGHT_THRESHOLD && room.width > CENTRAL_ROOM_WIDTH_THRESHOLD
        {
            ret_val.push(room.id);
        }
    }
    return ret_val;
}