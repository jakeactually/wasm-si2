use crate::types::Enemy;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn load(s: &str, f: &Closure<dyn Fn(Vec<u8>)>);
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

pub fn load_level(id: u8) -> std::io::Result<Vec<Enemy>> {
    
    let f = Closure::wrap(Box::new(|v| {
        alert(format!("{:?}", v).as_str());
    }) as Box<dyn Fn(Vec<u8>)>);

    load("/data/enemies/0.dat", &f);

    f.forget();

    let file = File::open(format!("data/levels/{}.dat", id))?;
    let bytes = file.bytes().collect::<std::io::Result<Vec<u8>>>()?; 

    let amount = bytes[0];
    let mut result = vec![];

    for i in 0..amount as u32 {
        let offset = i * 5;
        let view = bytes[(offset as usize + 1)..(offset as usize + 6)].to_vec();
        let enemy_id = view[3];
        let enemy_data = self.load_enemy(enemy_id)?;

        let enemy = Enemy {
            id: enemy_id,
            position: Vec2 {
                x: view[0] as i32 * 256 + view[1] as i32,
                y: view[2] as i32
            },
            dir: (view[4] as i32) - 1,
            data: enemy_data,
            explosion_frames: 2,
            anim_state: 0
        };

        result.push(enemy);
    }

    Ok(result)
}
