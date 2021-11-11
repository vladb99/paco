#[derive(Clone, Copy)]
pub struct Field {
    pub occupied: bool,
}
impl Field {
    pub fn new() -> Field {
        Field { occupied: false }
    }
    pub fn is_occupied(self) -> bool {
        self.occupied
    }
    pub fn set(&mut self, occupied: &bool) {
        self.occupied = *occupied;
    }
}
#[derive(Clone, Copy)]
pub struct Board {
    pub fields: [[Field; 13]; 13],
    _current: [usize; 2],
}
impl Board {
    pub fn new() -> Board {
        let fields = [[Field::new(); 13]; 13];
        Board {
            fields: fields,
            _current: [0, 0],
        }
    }
    pub fn set_index(&mut self, index: usize, occupied: bool) {
        self.fields[(index / 13) as usize][index % 13].set(&occupied)
    }
    pub fn get_index(self, index: usize) -> Field {
        self.fields[(index / 13) as usize][index % 13]
    }
    pub fn get(self, x: usize, y: usize) -> Field {
        self.fields[x][y]
    }
    pub fn set(&mut self, x: usize, y: usize, occupied: bool) {
        self.fields[x][y].set(&occupied)
    }
}
