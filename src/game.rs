
use cursive::vec::Vec2;
use rand::{thread_rng, Rng};

#[derive(Clone, Copy)]
pub struct Options {
    pub size : Vec2,
    pub n_alive : usize
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub alive : bool
}

pub struct Board {
    pub size : Vec2,
    pub cells : Vec<Cell>
}

impl Board {
    pub fn new(options: Options) -> Self {
        let n_cells = options.size.x * options.size.y;

        if options.n_alive > n_cells {
            return Board::new(
                Options{
                    size: options.size,
                    n_alive: n_cells
                }
            )
        }

        let mut board = Board{
            size: options.size,
            cells: vec![Cell{alive: false}; n_cells]
        };

        for _ in 0..options.n_alive {
            let i = loop {
                let i = thread_rng().gen_range(0, n_cells);

                if board.cells[i].alive {
                    continue;
                }

                break i;
            };

            board.cells[i].alive = true;
        }

        return board
    }

    pub fn neighbors(&self, pos: Vec2) -> Vec<Vec2> {
        let pos_min = pos.saturating_sub((1, 1));
        let pos_max = (pos + (2, 2)).or_min(self.size);
        (pos_min.x..pos_max.x)
            .flat_map(|x| (pos_min.y..pos_max.y).map(move |y| Vec2::new(x, y)))
            .filter(|&p| p != pos)
            .collect()
    }

    pub fn cell_pos(&self, i :usize) -> Vec2 {
        Vec2{
            x: i % self.size.x,
            y: i / self.size.x,
        }
    }

    pub fn cell_index(&self, pos: Vec2) -> usize {
        pos.x + pos.y * self.size.x
    }

    pub fn update(&mut self) {
        let n_cells = self.size.x * self.size.y;
        let last_cells =  self.cells.to_vec();
        self.cells = vec![Cell{alive: false}; n_cells];
        for (i,  _) in last_cells.iter().enumerate() {
            let mut alive_count = 0;
            for (_, pos) in self.neighbors(self.cell_pos(i)).iter().enumerate() {
                let idx = self.cell_index(*pos);
                if last_cells[idx].alive {
                    alive_count+=1;
                }
            }
            if last_cells[i].alive {
                if alive_count >= 2 && alive_count <= 3 {
                    self.cells[i].alive = true;
                }
            } else {
                if alive_count == 3 {
                    self.cells[i].alive = true;
                }
            }
        }
    }
}
