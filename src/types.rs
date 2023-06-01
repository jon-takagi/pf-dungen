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
    pub purpose: Option::<String>,
    pub encounter: Option::<Encounter>
}

impl Room {

    pub fn bottom_right(&self) -> (u32, u32)
    {
        return (self.top_left.0 + self.width, self.top_left.1 + self.height);
    }

    pub fn center_point(&self) -> (u32, u32)
    {
        return ((self.top_left.0 + (self.width / 2)), (self.top_left.1 + (self.height / 2)));
        // return ((self.top_left.0 + self.width / 2) as i32, (self.top_left.1 - self.height / 2) as i32);
    }

    pub fn overlaps(&self, other:&Room) -> bool 
    {
        if other.top_left.1 > self.bottom_right().1
        {
            // other is fully right of self
            return false;
        }
        if other.bottom_right().1 < self.top_left.1 
        {
            // other is fully left of self 
            return false;
        }
        if other.top_left.0 > self.bottom_right().0
        {
            // other is fully below self
            return false;
        }
        if other.bottom_right().0 < self.top_left.0
        {
            // other is fully above self
            return false;
        }
        return true;
    }

    pub fn contains(&mut self, point:(u32, u32)) -> bool
    {
        // point is below top edge 
        if self.top_left.1 < point.1 
        {
            // point is above bottom edge
            if point.1 < self.bottom_right().1
            {
                // point is right of left edge
                if self.top_left.0 < point.0
                {
                    // point is left of right edge
                    if point.0 < self.bottom_right().0
                    {
                        return true;
                    }
                }
            }
        }
        return false; 
    }

    pub fn separate_by(&mut self, velocity:(i32, i32), dungeon_width: u32, dungeon_height:u32) -> bool // return false if moving by velocity would hit a wall
    {
        let mut ret_val:bool = true;
        let mut tmp: i32 = (self.top_left.0 as i32) + velocity.0;
        if tmp < 0
        {
            tmp = 0;
            ret_val = false;
        }
        if tmp > dungeon_width as i32
        {
            tmp = dungeon_width as i32;
            ret_val = false;
        }
        self.top_left.0 = tmp as u32; // apply velocity change

        tmp = (self.top_left.1 as i32) + velocity.1;
        if tmp < 0
        {
            tmp = 0;
            ret_val = false;
        }
        if tmp > dungeon_height as i32
        {
            tmp = dungeon_height as i32;
            ret_val = false;
        }
        self.top_left.1 = tmp as u32;
        return ret_val;
    }

    pub fn is_inside_dungeon(&self) -> bool
    {
        if  self.bottom_right().0 > DUNGEON_WIDTH ||
            self.bottom_right().1 > DUNGEON_HEIGHT ||
            self.top_left.0 < 0 || 
            self.top_left.1 < 0
        {
            return false;
        }
        return true;
    }
}

// pub const EMPTY_ENCOUNTER:Encounter = Encounter {level:0, xp_award: 0, loot:vec![], monsters:vec![]};
pub const DUNGEON_WIDTH:u32                 = 300;
pub const DUNGEON_HEIGHT:u32                = 300;