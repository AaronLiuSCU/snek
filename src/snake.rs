use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;


const SNAKE_COLOR: Color = [0.00,0.00,0.00,1.0];
const SNAKE_HEAD_COLOR: Color = [1.0, 1.0, 1.0, 1.0];
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction{
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    x:i32,
    y:i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
    addlength: i32,
    invincibility_timer: i32,
}

impl Snake {
    pub fn new(x:i32, y:i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block {x: x+2, y,});
        body.push_back(Block {x:x+1, y,});
        body.push_back(Block {x,y,});
        Snake {
            direction: Direction::Right,
            body,
            tail: None,
            addlength: 0,
            invincibility_timer: 0,
        }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        let mut x = 0;
        for block in &self.body {
            if x == 0 {
                draw_block(SNAKE_HEAD_COLOR, block.x, block.y, con, g);
                x = x+1;
            }
            else {draw_block(SNAKE_COLOR, block.x, block.y, con, g);}
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.direction{
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1
            },
            Direction::Down => Block {
                x:last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            }
        };
        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }
        match moving_dir {
            Direction::Up => (head_x, head_y-1),
            Direction::Down => (head_x, head_y+1),
            Direction::Left => (head_x-1, head_y),
            Direction::Right => (head_x+1, head_y),
        }
    }

    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    pub fn overlap_tail(&self, x:i32, y:i32) -> bool {
        if self.is_invincible() {return false} else {()}
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }
            ch += 1;
            if ch == self.body.len()-1{
                break;
            }
        }
        return false;
    }

    pub fn draw_extend_tail(&mut self) {
        if self.addlength > 0 {self.addlength = self.addlength - 1; self.restore_tail();}
    }

    pub fn extend_tail(&mut self, x:i32) {
        self.addlength = self.addlength + x;
    }

    pub fn items_overlap_tail(&self, x:i32, y:i32) -> bool{
        let snake_end = self.body.back().unwrap();
        if snake_end.x == x && snake_end.y == y {return true;} else {()}
        match &self.tail {
            Some(blk) => if x == blk.x && y == blk.y {return true;} else {return false;},
            None => return false,
        }
    }

    pub fn increase_invincibility_timer(&mut self, x:i32) {
        self.invincibility_timer = self.invincibility_timer + x;
    }

    pub fn decrease_invincibility_timer(&mut self) {
        if self.invincibility_timer > 0 {self.invincibility_timer = self.invincibility_timer - 1;}
        else {()}
    }

    pub fn is_invincible(&self) -> bool {
        self.invincibility_timer > 0
    }
    
}