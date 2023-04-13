#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use rand::Rng;
use rand_distr::{Normal, Distribution};
mod types;
mod renderer;
use crate::types::{Room, Monster, Encounter, EMPTY_ENCOUNTER};
use petgraph::graph::*;
// This tool uses trademarks and/or copyrights owned by Paizo Inc., used under Paizo's Community Use Policy (paizo.com/communityuse). 
// We are expressly prohibited from charging you to use or access this content. 
// This tool is not published, endorsed, or specifically approved by Paizo. 
// For more information about Paizo Inc. and Paizo products, visit paizo.com.

const NUM_ROOMS:i32                     = 100;
const DUNGEON_WIDTH:u32                 = 100;
const DUNGEON_HEIGHT:u32                = 100;
const ROOM_WIDTH_MEAN:f64               = 6.0;
const ROOM_WIDTH_STD_DEV:f64            = 3.0; 
const ROOM_HEIGHT_MEAN:f64              = 8.0;
const ROOM_HEIGHT_STD_DEV:f64           = 3.0;
const ATTEMPT_THRESHOLD:i32             = -1; // keep trying until i break
const CENTRAL_ROOM_WIDTH_THRESHOLD:f64  = 1.25;
const CENTRAL_ROOM_HEIGHT_THRESHOLD:f64 = 1.25;
fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let mut rooms:Vec<Room> = Vec::new();

    let mut rng = rand::thread_rng();
    let height_dist = Normal::<f64>::new(ROOM_HEIGHT_MEAN, ROOM_HEIGHT_STD_DEV).unwrap();
    let width_dist = Normal::<f64>::new(ROOM_WIDTH_MEAN, ROOM_WIDTH_STD_DEV).unwrap();

    let mut room_index = 0;
    let mut failed_attempts = 0;

    while (rooms.len() as i32) < NUM_ROOMS
    {
        if ATTEMPT_THRESHOLD > 0 && failed_attempts == ATTEMPT_THRESHOLD
        {
            println!("attempt threshold reached");
            break;
        }
        let h:u32 = rng.sample(height_dist).round() as u32;
        let w:u32 = rng.sample(width_dist).round() as u32;

        let x:u32 = rng.gen_range(0..DUNGEON_WIDTH);
        let y:u32 = rng.gen_range(0..DUNGEON_HEIGHT);

        // do not create rooms that extend past the edges of the dungeon.

        if h > 1 && w > 1 && y > h
        {
            let mut new_room = Room {
                id: room_index,
                height: h,
                width: w,
                top_left: (x, y),
                purpose: "none".to_string(),
                encounter: EMPTY_ENCOUNTER
            };
    
            let mut overlaps:bool = false;
            for room in rooms.iter()
            {
                if overlaps 
                {
                    break;
                }
                overlaps = overlaps || new_room.overlaps(room);
            }            
            if !overlaps
            {
                rooms.push(new_room);
                failed_attempts = 0;
                room_index += 1;
            }
        }
        failed_attempts += 1;
    }

    // let goblin1 = Monster{name: "goblin1".to_string(), level:-1, aon_link:"https://2e.aonprd.com/Monsters.aspx?ID=232".to_string()};
    // let goblin2 = Monster{name: "goblin2".to_string(), level:-1, aon_link:"https://2e.aonprd.com/Monsters.aspx?ID=232".to_string()};
    // let goblin3 = Monster{name: "goblin3".to_string(), level:-1, aon_link:"https://2e.aonprd.com/Monsters.aspx?ID=232".to_string()};
    // let goblin4 = Monster{name: "goblin4".to_string(), level:-1, aon_link:"https://2e.aonprd.com/Monsters.aspx?ID=232".to_string()};
    // let goblin_ambush = Encounter {level:1, xp_award:60, loot:vec!["13gp".to_string()], monsters:vec![goblin1, goblin2, goblin3, goblin4]};
    // separate_rooms(DUNGEON_WIDTH, DUNGEON_HEIGHT, &mut rooms);

    let mut central_ids = find_central_rooms(&mut rooms);
    for room in &rooms 
    {
        let id = room.id;
        let h = room.height;
        let w = room.width;
        let top_left = room.top_left;
        println!("room {0} top_left at ({3}, {4}) with height {1} and width {2}", id, h, w, top_left.0, top_left.1);
    }
    renderer::render(DUNGEON_WIDTH, DUNGEON_HEIGHT, &mut rooms, "dungeon.png".to_string());
}

fn separate_rooms(dungeon_width:u32, dungeon_height:u32,rooms: &mut Vec<Room>)
{
    let mut i = 0;
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
                let difference = (second_room.center_point().0 - first_room.center_point().0, second_room.center_point().1 - first_room.center_point().1);
                let length = ((difference.0.pow(2) + difference.1.pow(2)) as f64).sqrt();
                let x_velocity = (difference.0 as f64/ length).round() as i32;
                let y_velocity = (difference.1 as f64/ length).round() as i32;

                drop(first_room);
                drop(second_room);
                let mutable_room:&mut Room = &mut rooms[id_1];
                mutable_room.separate_by((x_velocity, y_velocity), dungeon_width, dungeon_height);
                let mutable_room:&mut Room = &mut rooms[id_2];
                mutable_room.separate_by((-1 * x_velocity, -1 * y_velocity), dungeon_width, dungeon_height);
                // if mutable_room.top_left.0 + x_velocity < dungeon_width
                // {
                //     mutable_room.top_left.0 += x_velocity;
                // } else
                // {
                //     mutable_room.top_left.0 = dungeon_width;
                // }
                // if mutable_room.top_left.1 + y_velocity < dungeon_height
                // {
                //     mutable_room.top_left.1 += y_velocity;
                // } else 
                // {
                //     mutable_room.top_left.1 = dungeon_height;
                // }
                // drop(mutable_room);
                // let mut mutable_room:&Room = &mut rooms[id_2];

                // if mutable_room.top_left.0 > x_velocity
                // {
                //     mutable_room.top_left.0 -= x_velocity;
                // } else
                // {
                //     mutable_room.top_left.0 = 0;
                // }
                // if mutable_room.top_left.1 > y_velocity
                // {
                //     mutable_room.top_left.1 -= y_velocity;
                // } else
                // {
                //     mutable_room.top_left.1 = 0;
                // }
            }
        }
        if i % 10 == 0
        {
            let mut filename = i.to_string().to_string();
            filename.push_str(".png");
            renderer::render(dungeon_width, dungeon_height, rooms, filename);
        }
        if !any_rooms_overlap(rooms)
        {
            break;
        }
        i += 1;
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
                overlaps = overlaps || rooms.get(first_index).unwrap().overlaps(rooms.get(second_index).unwrap());
            }
        }
    }
    return overlaps;
}

fn find_central_rooms(rooms: &mut Vec<Room>) -> Vec<u32>
{
    let mut ret_val = Vec::<u32>::new();
    for room in &mut *rooms
    {
        if room.height > (ROOM_HEIGHT_MEAN * CENTRAL_ROOM_HEIGHT_THRESHOLD).round() as u32 && room.width > (ROOM_WIDTH_MEAN * CENTRAL_ROOM_WIDTH_THRESHOLD).round() as u32
        {
            ret_val.push(room.id);
        }
    }
    return ret_val;
}