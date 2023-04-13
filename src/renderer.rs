use crate::types::Room;
use image;
use image::{ImageBuffer, Rgb};
pub fn render(dungeon_width:u32, dungeon_height:u32, rooms:&mut Vec<Room>, filename: String)
{
    let tile_size_px = 10;
    println!("rendering {0} rooms", rooms.len());
    
    let mut image:ImageBuffer<Rgb<u8>, _> = ImageBuffer::new(dungeon_width * tile_size_px, dungeon_height * tile_size_px);
    for pixel_x in 0..image.width()
    {
        for pixel_y in 0..image.height()
        {
            let mut in_room:bool = false;
            for room in &mut *rooms
            {
                let dung_x = pixel_x / tile_size_px;
                let dung_y = pixel_y / tile_size_px;
                in_room = in_room || (*room).contains((dung_x, dung_y));
            }
            if in_room
            {
                image.put_pixel(pixel_x, pixel_y, Rgb([255, 255, 255]));
            }
        }
    }
    image.save(filename).unwrap();
}