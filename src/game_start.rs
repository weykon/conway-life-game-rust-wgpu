pub struct Gaming {
    pub blocks: Vec<Vec<Block>>,
    pub entities: Vec<Location>,
}
impl Gaming {
    fn new(w: i32, h: i32) -> Self {
        Gaming {
            blocks: vec![vec![Block::Empty; w as usize]; h as usize],
            entities: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub enum Block {
    Empty,
    Entity(Location),
    Terrain(TerrainType),
}

#[derive(Clone)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone)]
pub enum TerrainType {
    Normal,
    Water,
    Wall,
    Sand,
}

impl Block {
    fn set(&mut self, gaming: &mut Gaming) {
        match self {
            Block::Empty => {}
            Block::Entity(location) => {
                gaming.entities.push(location.clone());
                gaming.blocks[location.y as usize][location.x as usize] = Block::Empty;
            }
            Block::Terrain(terrain_type) => match terrain_type {
                TerrainType::Normal => {}
                TerrainType::Water => {}
                TerrainType::Wall => {}
                TerrainType::Sand => {}
            },
        }
    }
}
