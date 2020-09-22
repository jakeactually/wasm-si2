use crate::types::{Vec2};

pub fn uncompress(data: Vec<u8>) -> Vec<u8> {
    let mut result = vec![];

    for byte in data.iter() {
        for i in 0..8 {
            result.push((byte >> (7 - i)) & 1);
        }
    }

    result
}

pub fn intersect(start1: Vec2, size1: Vec2, start2: Vec2, size2: Vec2) -> bool {
    !(start1.x > start2.x + size2.x - 1 || start1.y > start2.y + size2.y - 1 || start1.x + size1.x - 1 < start2.x || start1.y + size1.y - 1 < start2.y)
}

pub fn vec_map<T: Clone, U, G: Fn(U), F: Fn(T, G)>(v: Vec<T>, f: F, g: impl Fn(Vec<U>)) {
    vec_map_rec(v, f, g, vec![]);
}

fn vec_map_rec<T: Clone, U, G: Fn(U), F: Fn(T, G)>(v: Vec<T>, f: F, g: impl Fn(Vec<U>), accum: Vec<U>) {
    if v.len() == 0 {
        g(accum);
    } else {
        let x = v[0];
        let xs = v[1..].to_vec();
        
        let h: G = |y: U| {

        };

        f(x, unimplemented!());
    }
}
