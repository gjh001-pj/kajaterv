use web_sys::HtmlInputElement;
use yew::prelude::*;
use gloo::console::log;
//use wasm_bindgen::JsCast;

pub mod dirs {
    pub const UP    : u8 = 0b0001;
    pub const RIGHT : u8 = 0b0010;
    pub const DOWN  : u8 = 0b0100;
    pub const LEFT  : u8 = 0b1000;
    pub const ALL   : u8 = 0b1111;
}

#[derive(Debug, PartialEq)]
pub enum EditMode {
    Edit,
    View,
}

#[derive(Debug, PartialEq)]
pub struct TableFocusNavigator {
    pub refs: Vec<Vec<NodeRef>>,
    pub dirs: Vec<Vec<u8>>,
    pub mode: EditMode,
    pub rows: usize,
    pub cols: usize,
}

impl TableFocusNavigator {
    pub fn new(rows: usize, cols: usize) -> TableFocusNavigator {
        let mut dirs: Vec<Vec<u8>> = (0..rows).map(|_|
            (0..cols).map(|_| dirs::ALL).collect()
        ).collect();
        if rows > 0 {
            for x in dirs.first_mut().unwrap() {
                *x &= dirs::ALL & !dirs::UP;
            }
            for x in dirs.last_mut().unwrap() {
                *x &= dirs::ALL & !dirs::DOWN;
            }
        }
        if cols > 0 {
            for row in dirs.iter_mut() {
                *row.first_mut().unwrap() &= dirs::ALL & !dirs::LEFT;
                *row.last_mut().unwrap() &= dirs::ALL & !dirs::RIGHT;
            }
        }

        let refs = (0..rows).map(|_| {
            (0..cols).map(|_| NodeRef::default()).collect()
        }).collect();

        TableFocusNavigator {
            refs,
            dirs,
            mode: EditMode::Edit,
            rows,
            cols,
        }
    }

    pub fn build(&mut self, rows: usize, cols: usize) {
        *self = TableFocusNavigator::new(rows, cols);
    }

    pub fn set_edit(&mut self) {
        self.mode = EditMode::Edit
    }

    pub fn set_view(&mut self) {
        self.mode = EditMode::View
    }

    fn can_go(&self, direction: u8, row: usize, col: usize) -> bool {
        self.dirs[row][col] & direction > 0
    }

    pub fn handle_key(&mut self, row: usize, col: usize, e: KeyboardEvent) {
        //log!(format!("row: {}, col: {}, e.key(): {}, self.mode: {:?}", row, col, e.key(), self.mode));
        if let EditMode::Edit = self.mode {
            if e.key() == "Enter" {
                self.mode = EditMode::View;
            }
            return;
        }

        let (mut new_row, mut new_col) = (row, col);
        match e.key().as_str() {
            "ArrowUp"    if self.can_go(dirs::UP,    row, col) => 
                new_row -= 1,
            "ArrowRight" if self.can_go(dirs::RIGHT, row, col) => 
                new_col += 1,
            "ArrowDown"  if self.can_go(dirs::DOWN,  row, col) => 
                new_row += 1,
            "ArrowLeft"  if self.can_go(dirs::LEFT,  row, col) => 
                new_col -= 1,
            "Enter" => {
                self.mode = EditMode::Edit; return
            },
            _ => return,
        }

        if let Some(input) = self.refs[new_row][new_col].cast::<HtmlInputElement>() {
            input.focus().ok();
            e.prevent_default();
        }
    }
}

#[test]
fn test1() {
    assert_eq!((1..2).collect::<Vec<i32>>(), vec![1]);
}