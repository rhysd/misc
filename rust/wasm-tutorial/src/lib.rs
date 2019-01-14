extern crate cfg_if;
extern crate fixedbitset;
extern crate js_sys;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use fixedbitset::FixedBitSet;
use js_sys::Math;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for drow in [self.height - 1, 0, 1].iter().cloned() {
            for dcol in [self.width - 1, 0, 1].iter().cloned() {
                if drow == 0 && dcol == 0 {
                    continue;
                }
                let row = (row + drow) % self.height;
                let col = (column + dcol) % self.width;
                let idx = self.get_index(row, col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

/// Public methods exposed to JavaScript by wasm_bindgen attribute
#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            // Note: true means alive, false means dead
            cells.set(i, Math::random() < 0.2);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                const ALIVE: bool = true;
                const DEAD: bool = false;
                let next_cell = match (cell, live_neighbors) {
                    (ALIVE, 2) | (ALIVE, 3) => ALIVE, // Rule 2
                    (ALIVE, _) => DEAD,               // Rule 1 and rule 3
                    (DEAD, 3) => ALIVE,               // Rule 4
                    (DEAD, _) => DEAD,
                };
                next.set(idx, next_cell);
            }
        }
        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
}
