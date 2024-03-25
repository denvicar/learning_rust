use bracket_lib::{color::{BLACK, RED}, random::RandomNumberGenerator, terminal::{to_cp437, BTerm}};

use crate::{game::SCREEN_HEIGHT, player::Player};

pub struct Obstacle {
    pub x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    pub fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20-score),
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in self.gap_y+half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    pub fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size/2;
        (self.x == player.x) && 
        ((player.y > self.gap_y+half_size) || (player.y < self.gap_y-half_size))
    }
}