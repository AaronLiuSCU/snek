use piston_window::*;
use piston_window::types::Color;
use keyboard::Key;
use rand::{thread_rng, Rng};

use crate::snake::{Direction, Snake};
use crate::draw::draw_rectangle;
use crate::items::{Items, Ingame};
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.4;
const RESTART_TIME:f64 = 1.0;

pub struct Game {
    snake: Snake,
    width: i32,
    height: i32,
    game_over: bool,
    waiting_time: f64,
    ingame: Ingame,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game{
       Game {
           snake: Snake::new(2,2),
           waiting_time: 0.0,
           width,
           height,
           game_over: false,
           ingame: Ingame::new(width, height),
       }
    }
    pub fn key_pressed(&mut self, key:Key) {
        if self.game_over {
            return;
        }
        let dir = match key{
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None
        };
        if dir.unwrap() == self.snake.head_direction().opposite() || dir.unwrap() == self.snake.head_direction(){
            return;
        }
        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d){
        self.snake.draw(con, g);
        self.ingame.draw(con,g);
        draw_rectangle(BORDER_COLOR, 0,0, self.width, 1, con,g);
        draw_rectangle(BORDER_COLOR, 0, self.height-1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0,0,1,self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width-1, 0,1,self.height, con, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0,0, self.width, self.height, con, g);
        }
    }
    pub fn update( &mut self, delta_time:f64) {
        self.waiting_time += delta_time;
        let mut rng = thread_rng();
        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }
        if self.waiting_time > MOVING_PERIOD {
            let color:Color = [rng.gen_range(0.0..=1.0), rng.gen_range(0.0..=1.0), rng.gen_range(0.0..=1.0),1.0];
            self.ingame.set_superfruit_color(color);
            self.update_snake(None);
        }
    }
    fn check_encounter(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        let (v1, v2) = self.ingame.encounter_item(head_x, head_y);
        match v1 {
            Some(item) => match item {
                Items::Fruit(_) => {self.ingame.remove_fruit(v2); self.ingame.add_item(&self.snake); self.snake.extend_tail(1)},
                Items::Pole(_) => if !self.snake.is_invincible() {self.game_over = true;} else {self.ingame.remove_fruit(v2)},
                Items::Doublefruit(_) => {self.ingame.remove_fruit(v2); self.ingame.add_item(&self.snake); self.snake.extend_tail(2)},
                Items::Superfruit(_) => {self.ingame.remove_fruit(v2); self.ingame.add_item(&self.snake); self.snake.increase_invincibility_timer(5)},
            }
            None => (),
        }

    }
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }
        next_x > 0 && next_y > 0 && next_x < (self.width -1) && next_y < (self.height - 1)
    }


    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.snake.decrease_invincibility_timer();
            self.check_encounter();
            self.snake.draw_extend_tail();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2,2);
        self.waiting_time = 0.0;
        self.game_over = false;
        self.ingame = Ingame::new(self.width, self.height);
    }
}