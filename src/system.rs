use leptos::{create_rw_signal, RwSignal, SignalGet, SignalSet};

#[derive(Clone)]
pub struct Cell {
    pub id: usize,
    pub stroke: RwSignal<char>,
    done: RwSignal<bool>,
}

impl Cell {
    pub fn new(index: usize) -> Self {
        Cell {
            id: index,
            stroke: create_rw_signal(' '),
            done: create_rw_signal(false),
        }
    }
    pub fn make_stroke(&mut self, ch: char) -> bool {
        if self.done.get() {
            return false;
        }

        self.stroke.set(ch);
        self.done.set(true);
        true
    }
    pub fn reset(&mut self) {
        self.done.set(false);
        self.stroke.set(' ')
    }
}

#[derive(Clone)]
pub struct Grid(Vec<Cell>, usize, pub bool);

impl Grid {
    pub fn new() -> Self {
        let mut vecs: Vec<Cell> = Vec::new();
        for i in 0..9 {
            vecs.push(Cell::new(i));
        }
        Grid(vecs, 0, false)
    }
    pub fn row(&self, index: usize) -> Vec<Cell> {
        self.0
            .iter()
            .skip(3 * (index - 1))
            .map_while(|x| {
                if x.id < 3 * index {
                    Some(x.clone())
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn cell_stroke(&mut self, index: usize, stroke: char) -> bool {
        let stroke = !self.2 && self.0[index].make_stroke(stroke);
        if self.1 + 1 >= 5 {
            // YOu cant win under 5 turns
            self.outcome();
        }
        stroke
    }

    #[allow(clippy::if_same_then_else)]
    pub fn outcome(&mut self) {
        let rows = (1..=3)
            .map(|i| {
                self.row(i)
                    .iter()
                    .map(|c| c.stroke.get())
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();

        let (r1, r2, r3) = (&rows[0], &rows[1], &rows[2]);

        // Row wise
        if (0..3)
            .filter(|i| {
                rows[*i]
                    .iter()
                    .take_while(|x| *x != &' ' && &rows[*i][0] == *x)
                    .count() == 3
            })
            .count()
            > 0 
        { 
            self.2 = true 
        }

        // Column wise
        else if r1
            .iter()
            .zip(r2.iter())
            .zip(r3.iter())
            .filter(|((x, y), z)| *x != &' ' && x == y && y == z)
            .count() 
            > 0
        {
            self.2 = true;
        }

        // Diagonals
        else if r1[0] != ' ' && r1[0] == r2[1] && r1[0] == r3[2] {
            self.2 = true;
        }
        else if r3[0] != ' ' && r3[0] == r2[1] && r3[0] == r1[2] {
            self.2 = true;
        }
    }

    pub fn update_turn(&mut self) {
        self.1 += 1;
    }
    pub fn get_turn(&self) -> usize {
        self.1
    }

    pub fn reset(&mut self) {
        for cell in &mut self.0 {
            cell.reset();
        }
        self.1 = 0;
        self.2 = false;
    }
}
