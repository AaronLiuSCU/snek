use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;
use rand::{thread_rng, Rng};

use crate::draw::draw_block;
use crate::snake::Snake;

const FRUIT_COLOR:Color = [0.80, 0.00,0.00,1.0];
const POLE_COLOR:Color = [0.00, 0.9, 0.2, 1.0];
const DOUBLEFRUIT_COLOR:Color = [1.0, 1.0, 0.00, 1.0];
#[derive(PartialEq)]
pub struct Fruitvars {
    x:i32,
    y:i32,
}

impl Fruitvars {
    pub fn draw_fruit(&self, con: &Context, g: &mut G2d) {
        draw_block(FRUIT_COLOR, self.x, self.y, con, g);
    }
}
#[derive(PartialEq)]
pub struct Polevars {
    x:i32,
    y:i32,
}


impl Polevars {
    pub fn draw_pole(&self, con: &Context, g: &mut G2d) {
        draw_block(POLE_COLOR, self.x, self.y, con, g);
    }
}


#[derive(PartialEq)]
pub struct DFruitvars {
    x:i32,
    y:i32,
}


impl DFruitvars {
    pub fn draw_doublefruit(&self, con: &Context, g: &mut G2d) {
        draw_block(DOUBLEFRUIT_COLOR, self.x, self.y, con, g);
    }
}

#[derive(PartialEq)]
pub struct SFruitvars {
    x:i32,
    y:i32,
    color: Color
}


impl SFruitvars 
{
    pub fn set_color(&mut self, color:Color) {
        self.color = color;
    }
    pub fn draw_superfruit(&self, con: &Context, g: &mut G2d) {
        /* 
        let mut rng = thread_rng();
        let random_color:Color = [rng.gen_range(0.0..=1.0), rng.gen_range(0.0..=1.0), rng.gen_range(0.0..=1.0),1.0];
        */
        draw_block(self.color, self.x, self.y, con, g);
    }
}


#[derive(PartialEq)]
pub enum Items {
    Fruit(Fruitvars),
    Pole(Polevars),
    Doublefruit(DFruitvars),
    Superfruit(SFruitvars),
}

pub struct Ingame {
    items: LinkedList<Items>,
    screen_width: i32,
    screen_height: i32,
}

impl Ingame {
    pub fn new(w:i32, h:i32) -> Ingame {
        let mut items: LinkedList<Items> = LinkedList::new();
        items.push_back(Items::Fruit(Fruitvars {x: 4, y: 6}));
        Ingame {items, screen_width:w, screen_height:h}
    }

    pub fn set_superfruit_color(&mut self, color:Color) {
        for item in &mut self.items {
            match item {
                Items::Fruit(_) => (),
                Items::Pole(_) => (),
                Items::Doublefruit(_) => (),
                Items::Superfruit(d) => d.set_color(color),
            }
        }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for item in &self.items {
            match item {
                Items::Fruit(a) => a.draw_fruit(con, g),
                Items::Pole(b) => b.draw_pole(con, g),
                Items::Doublefruit(c) => c.draw_doublefruit(con, g),
                Items::Superfruit(d) => d.draw_superfruit(con, g),
            }
        }
    }

    fn find_location(&self, snake:&Snake) -> (i32, i32) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..=self.screen_width -1);
        let mut new_y = rng.gen_range(1..=self.screen_height -1);


        while snake.overlap_tail(new_x, new_y) || self.overlap_item(new_x, new_y) || snake.items_overlap_tail(new_x, new_y){
            new_x = rng.gen_range(1..=self.screen_width-1);
            new_y = rng.gen_range(1..=self.screen_height-1);
        }
        (new_x, new_y)
    }

    pub fn add_item(&mut self, snake:&Snake) {
        let mut rng1 = thread_rng();
        let (new_x, new_y) = self.find_location(snake);
        let gen_fruit = rng1.gen_range(0..=9);
        if gen_fruit >= 8 {self.items.push_back(Items::Doublefruit(DFruitvars {x: new_x, y: new_y}));}
        else if gen_fruit >= 2 {self.items.push_back(Items::Superfruit(SFruitvars {x: new_x, y: new_y, color: [rng1.gen_range(0.0..=1.0), rng1.gen_range(0.0..=1.0), rng1.gen_range(0.0..=1.0),1.0]}));}
        else {self.items.push_back(Items::Fruit(Fruitvars {x: new_x, y: new_y}));}
        println!("Fruit added at {}, {}", new_x, new_y);
        
        let gen_pole = rng1.gen_range(0..=2);
        for _n in 1..=gen_pole {
            let (new_x2, new_y2) = self.find_location(snake);
            println!("Pole added at {}, {}", new_x2, new_y2);
            self.items.push_back(Items::Pole(Polevars {x: new_x2, y: new_y2}))
        }
    }

    fn overlap_item(&self, item_x:i32, item_y:i32) -> bool {
        for item in &self.items {
            match item {
                Items::Fruit(a) => if item_x == a.x && item_y == a.y {return true},
                Items::Pole(b) => if item_x == b.x && item_y == b.y {return true},
                Items::Doublefruit(c) => if item_x == c.x && item_y == c.y {return true},
                Items::Superfruit(d) => if item_x == d.x && item_y == d.y {return true},
            }
        }
        false
    }
    pub fn remove_fruit(&mut self, x:usize){
        let mut split_list:LinkedList<Items> = self.items.split_off(x);
        split_list.pop_front();
        self.items.append(&mut split_list);  
    }

    pub fn encounter_item(&mut self, snake_x:i32, snake_y:i32) -> (Option<Items>,usize) {
        let (mut returnoption, mut returnindex):(Option<Items>, usize) = (None, 0);

        for item in &self.items {
            match item {
                Items::Fruit(a) => if snake_x == a.x && snake_y == a.y {
                    returnoption = Some(Items::Fruit(Fruitvars {x: a.x, y: a.y}));
                    break;
                } else {returnindex = returnindex+1;},
                Items::Pole(b) => if snake_x == b.x && snake_y == b.y {
                    returnoption = Some(Items::Pole(Polevars {x: b.x, y: b.y}));
                    break;
                } else {returnindex = returnindex+1;},
                Items::Doublefruit(c) => if snake_x == c.x && snake_y == c.y {
                    returnoption = Some(Items::Doublefruit(DFruitvars {x: c.x, y: c.y}));
                    break;
                } else {returnindex = returnindex+1;},
                Items::Superfruit(d) => if snake_x == d.x && snake_y == d.y {
                    returnoption = Some(Items::Superfruit(SFruitvars {x: d.x, y: d.y, color: d.color}));
                    break;
                } else {returnindex = returnindex+1;},
            }
        }
        (returnoption,returnindex)
    }
}