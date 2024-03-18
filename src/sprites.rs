const TOP: i32 = 0;
const BOTTOM: i32 = 63;
const LEFT: i32 = 0;
const RIGHT: i32 = 127;

pub struct Ball {
    x: i32,
    y: i32,
    ix: i32,
    iy: i32,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            x: LEFT,
            y: TOP,
            ix: 1,
            iy: 1,
        }
    }

    pub fn update(&mut self) {
        self.x += self.ix;
        self.y += self.iy;
        if !(LEFT + 1..=RIGHT - 1).contains(&self.x) {
            self.ix = -self.ix;
        }
        if !(TOP + 1..=BOTTOM - 1).contains(&self.y) {
            self.iy = -self.iy;
        }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }
}
