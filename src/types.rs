pub struct Monster {
    pub name: String,
    pub level: i32,
    pub aon_link: String
}

pub struct Encounter{
    pub level: i32,
    pub xp_award: u32,
    pub loot: Vec<String>,
    pub monsters: Vec <Monster>
}

pub struct Room{
    pub id: u32,
    pub height: u32,
    pub width: u32,
    pub top_left: (u32, u32),
    pub purpose: String,
    pub encounter: Encounter
}

impl Room {

    pub fn bottom_right(&self) -> (u32, u32)
    {
        return (self.top_left.0 + self.width, self.top_left.1 - self.height);
    }

    pub fn center_point(&self) -> (u32, u32)
    {
        return ((self.top_left.0 + self.width / 2), (self.top_left.1 - self.height / 2));
        // return ((self.top_left.0 + self.width / 2) as i32, (self.top_left.1 - self.height / 2) as i32);
    }

    pub fn overlaps(&self, other:&Room) -> bool 
    {
        if self.top_left.0 > other.bottom_right().0 || other.top_left.0 > self.bottom_right().0
        {
            return false;
        }
        if self.bottom_right().1 > other.top_left.1 || other.bottom_right().1 > self.top_left.1
        {
            return false;
        }
        return true;
    }

    pub fn contains(&mut self, point:(u32, u32)) -> bool
    {
        return point.0 > self.top_left.0 && point.0 < self.bottom_right().0 && point.1 > self.bottom_right().1 && point.1 < self.top_left.1;
    }
    pub fn separate_by(&mut self, velocity:(i32, i32), dungeon_width: u32, dungeon_height:u32)
    {
        let mut tmp: i32 = (self.top_left.0 as i32) + velocity.0;
        tmp = core::cmp::max(0, tmp);
        tmp = core::cmp::min(tmp, dungeon_width as i32);
        self.top_left.0 = tmp as u32;
        tmp = (self.top_left.1 as i32) + velocity.1;
        tmp = core::cmp::max(0, tmp);
        tmp = core::cmp::min(tmp, dungeon_height as i32);
        self.top_left.1 = tmp as u32;
    }
}

pub const EMPTY_ENCOUNTER:Encounter = Encounter {level:0, xp_award: 0, loot:vec![], monsters:vec![]};