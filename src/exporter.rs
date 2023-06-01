use crate::types::Room;
use std::io::Write;

pub fn export_floor(rooms:&mut Vec<Room>, filename: &String)
{
    let mut data = json::JsonValue::new_object();
    std::fs::remove_file(filename);
    let columns = json::array![];
    data["columns"] = columns;
    data["doors"] = json::array![];
    
    let mut rects = json::JsonValue::new_array();
    for room in rooms
    {
        let mut rect = json::JsonValue::new_object();
        rect["x"] = room.top_left.0.into();
        rect["y"] = room.top_left.1.into();
        rect["h"] = room.height.into();
        rect["w"] = room.width.into();
        rects.push(rect);
    }

    data["rects"] = rects;
    let mut file = std::fs::File::open(filename).unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound
        {
            return std::fs::File::create(filename).unwrap();
        }
        else
        {
            panic!("{:?}", error);
        }
    });
    data["story"] = json::JsonValue::String("".to_string());
    data["title"] = json::JsonValue::String("".to_string());
    data["version"] = json::JsonValue::String("1.2.4b".to_string());
    data["water"] = json::JsonValue::new_array();

    println!("{}", data.dump());
    file.write(data.dump().as_bytes());
}