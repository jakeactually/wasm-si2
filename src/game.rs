use crate::types::{Game, WIDTH, HEIGHT, PLAYER_HEIGHT, G_PLAYER, G_PROTECTION_A1, Vec2, Context};
use crate::objects::{scenery_data, Graphics};

use wasm_bindgen::prelude::*;

impl Game {
    pub fn render(&mut self) {
        if self.game_over {
            self.render_text("game", 5, 5);
            self.render_text("over", 35, 5);
            return;
        }

        if self.player.protected() {
            let player = self.load_object(G_PROTECTION_A1 + self.player.protection / 2 % 2);
            self.render_object(&player, self.player.position.x - 2, self.player.position.y - 2);
        } else {
            let player = self.load_object(G_PLAYER);
            self.render_object(&player, self.player.position.x, self.player.position.y);
        }

        self.render_bar();

        for scenery in self.scenery.clone() {
            if self.scene_x + scenery.position.x < WIDTH as i32 {
                self.render_object(&scenery.model, self.scene_x + scenery.position.x, scenery.position.y);
            }
        }

        for enemy in self.enemies.clone() {
            if self.enemies_x + enemy.position.x < WIDTH as i32 {
                enemy.render(self);
            }
        }

        for shot in self.shots.clone().iter() {
            let bullet = shot.weapon_kind.clone().model(self);
            self.render_object(&bullet, shot.position.x, shot.position.y);
        }
    }
}
