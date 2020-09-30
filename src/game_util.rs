use crate::types::*;
use crate::objects::{get_static_objects, get_weapons, scenery_data};

//use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        crate::utils::set_panic_hook();

        Game {
            data :vec![],
            ready: false,
            
            screen: [[0; WIDTH as usize]; HEIGHT as usize],
            inverted: false,

            static_objects: get_static_objects().to_vec(),
            weapons: get_weapons().to_vec(),
            levels_data: crate::data::levels::levels(),
            objects_data: crate::data::objects::objects(),
            enemies_data: crate::data::enemies::enemies(),

            scenery: vec![],
            enemies: vec![],
            shots: vec![],

            is_playing: false,
            game_over: false,
            level: 0,
            time: 0,
            scene_x: 0,
            enemies_x: 0,

            player: Player {
                position: Vec2 { x: 3, y: 20 },
                lives: 3,
                protection: 0,
            },
            y_axis: Vec2 { x: 5, y: HEIGHT as i32 - PLAYER_HEIGHT as i32 },
            weapon: Weapon {
                amount: 3,
                kind: WeaponKind::Missile
            },
            score: 0,
        }
    }

    pub fn update(&mut self, _ctx: &Context) {
        self.clear();

        if self.game_over {
            return;
        }

        self.keyboard(_ctx);
        
        if !self.is_playing {
            if self.level == 5 {
                self.game_over = true;
                return;
            }
            
            self.inverted = self.level_data().inverted_color;
            self.enemies = self.load_level(self.level);

            if self.level_data().upper == 1 {
                self.y_axis = Vec2 { x: 0, y: HEIGHT as i32 - PLAYER_HEIGHT as i32 - 5 };
            }

            self.load_scenery();

            self.player.position = Vec2 { x: 3, y: 20 };
            self.scene_x = 0;
            self.enemies_x = 0;
            self.level += 1;

            self.is_playing = true;            
        }

        // Enemies

        if self.enemies.len() == 0 {
            self.player.position.x += 1;
        }

        if self.player.position.x > WIDTH as i32 + 20 {
            self.is_playing = false;
            return;
        }

        let enemies_x = self.enemies_x;

        self.enemies = self
            .enemies
            .clone()
            .into_iter()
            .filter(|e| e.active() && enemies_x + e.position.x > -20)
            .map(|e| e.tick(self))
            .collect::<Vec<_>>();

        // Shots

        let nearest_y = if self.enemies.len() > 0 {
            self.enemies[0].position.y
        } else {
            self.player.position.y
        };

        self.shots = self
            .shots
            .clone()
            .into_iter()
            .filter(|s| s.active && s.position.x < WIDTH as i32)
            .map(|s| s.tick(nearest_y))
            .collect();

        // The end

        if let Some(enemy) = self.enemies.last() {
            if self.enemies_x + enemy.position.x >= (WIDTH as i32 / 4) * 3 {
                self.scene_x -= 1;
            }
        }

        if self.player.protected() {
            self.player.protection -= 1;
        }

        self.time += 1;
        self.enemies_x -= 1;

        if self.player.lives == 0 {
            self.game_over = true;
        }
    }

    pub fn do_render(&mut self) {
        self.render();
    }

    pub fn screen_pointer(&mut self) -> *mut [[u8; WIDTH as usize]; HEIGHT as usize] {
        &mut self.screen
    }
}

impl Game {
    pub fn level_data(&self) -> SceneryData {
        scenery_data[self.level as usize].clone()
    }

    pub fn keyboard(&mut self, _ctx: &Context) {
        /*let position = &mut self.player.position;

        if keyboard::is_key_pressed(_ctx, KeyCode::Right) && position.x < WIDTH as i32 - PLAYER_WIDTH as i32 {
            position.x += 1;
        } else if keyboard::is_key_pressed(_ctx, KeyCode::Left) && position.x > 0 {
            position.x -= 1;
        } else if keyboard::is_key_pressed(_ctx, KeyCode::Up) && position.y > self.y_axis.x {
            position.y -= 1;
        } else if keyboard::is_key_pressed(_ctx, KeyCode::Down) && position.y < self.y_axis.y {
            position.y += 1;
        }
        
        if self.time % 6 == 0 {            
            use keyboard::is_key_pressed;
            use KeyCode::*;

            if is_key_pressed(_ctx, Space) {
                let position = Vec2 { x: position.x + 9, y: position.y + 3 };
                let shot = Shot { position, active: true, weapon_kind: WeaponKind::Standard, duration: 3 };
                self.shots.push(shot);
            } else if is_key_pressed(_ctx, LAlt) || is_key_pressed(_ctx, RAlt) {
                if self.weapon.amount > 0 {
                    self.weapon.amount -= 1;                    
                    let y = if self.weapon.kind == WeaponKind::Wall { 5 } else { position.y + 3 };

                    let position = Vec2 { x: position.x + 9, y: y };
                    let shot = Shot { position, active: true, weapon_kind: self.weapon.kind.clone(), duration: 3 };
                    self.shots.push(shot);
                }
            }
        }*/
    }

    pub fn load_scenery(&mut self) {
        self.scenery = vec![];

        let mut x = 0;
        //let mut rng = rand::thread_rng();

        if self.level > 0 {
            while x < 1600 {
                let sd = &scenery_data[self.level as usize];
                let n: u8 = 2;//rng.gen_range(sd.first_object, sd.first_object + sd.objects);
                let rock = self.load_object(n);
                let y = if self.level_data().upper == 1 { 0 } else { HEIGHT as i32 - rock.size.y };

                self.scenery.push(Scenery { position: Vec2 { x, y }, model: rock.clone() });
                x += rock.size.x;
            }
        }
    }
}
