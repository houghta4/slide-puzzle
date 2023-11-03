use std::usize;

use js_sys::{Array, Math::random};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str); // binds to js function

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn table(v: Vec<i32>);
}

#[wasm_bindgen]
pub fn is_solved(v: Vec<i32>) -> bool {
    !v.iter().enumerate().any(|(i, &e)| e != i as i32)
}

#[wasm_bindgen]
pub fn shuffle(v: Vec<usize>, size: usize) -> Array {
    if v.len() <= 1 {
        return v.into_iter().map(JsValue::from).collect();
    }
    let mut coords: Vec<(usize, i32, i32)> = v
        .iter()
        .enumerate()
        .map(|(index, &value)| (value, (index / size) as i32, (index % size) as i32))
        .collect();
    let mut empty_i: usize = v.len() - 1;
    for _ in 0..300 {
        let empty = coords[empty_i];
        let mut dr = empty.1;
        let mut dc = empty.2;

        if random() <= 0.5 {
            // move rows
            if random() <= 0.5 {
                dr += 1;
            } else {
                dr -= 1;
            }
        } else {
            // move cols
            if random() <= 0.5 {
                dc += 1;
            } else {
                dc -= 1;
            }
        }
        let i: usize = (dr * size as i32 + dc) as usize;
        if i < coords.len()
            && i != empty_i
            && valid_move(
                coords[i].1,
                coords[i].2,
                coords[empty_i].1,
                coords[empty_i].2,
            )
        {
            // swap indices and switch empty_i
            let tmp = coords[empty_i];
            coords[empty_i].0 = coords[i].0;
            coords[i].0 = tmp.0;
            empty_i = i;
        }
    }
    coords
        .iter()
        .map(|&(first, _, _)| JsValue::from(first))
        .collect()
}

fn valid_move(tile_x: i32, tile_y: i32, target_x: i32, target_y: i32) -> bool {
    let x_valid = (tile_x == target_x - 1) || (tile_x == target_x + 1);
    let y_valid = (tile_y == target_y - 1) || (tile_y == target_y + 1);
    (tile_x == target_x || tile_y == target_y) && (x_valid || y_valid)
}

#[wasm_bindgen]
pub fn shuffle2(v: &mut [usize]) -> Array {

    let mut empty_tile_pos = v.len() - 1;
    // fisher yates
    for i in (1..v.len()).rev() {
        let rng = js_sys::Math::random();
        let j = (rng * (i as f64 + 1.0)) as usize;
        v.swap(i, j);
        if i == v.len() - 1{
            empty_tile_pos = i;
        }
    }


    v.iter().map(|&e|JsValue::from(e)).collect()
}

/*
    To find if it is solvable, we need to use inversions. 
    Inversions are just instances where a tile precedes another tile with a lower value

    Checking solvability
    If grid is odd -> inversions must be even
    if grid is even and empty tile is in an odd row counting from the bottom -> inversions must be even
    if grid is even and empty tile is in an even row counting from the bottom -> inversions must be odd
 */
#[allow(dead_code)]
fn is_solvable(_inv: usize, _width: usize, _empty_tile_pos: usize) -> bool {

    true
}

#[allow(dead_code)]
fn count_inversions(v: &mut [i32]) -> usize {
    let mut invs = 0;
    let empty_tile = v.len() as i32;
    for i in 0..v.len() {
        if v[i] == empty_tile {
            continue;
        } 
        for j in i + 1..v.len() {
            if v[j] == empty_tile {
                continue;
            }
            if v[i] > v[j] {
                invs += 1;
            }
        }
    }

    invs
}

#[cfg(test)]
mod tests {
    use crate::*;


    #[test]
    fn test_inversions() {

        let a = count_inversions(&mut [8, 7, 5, 1, 9, 4, 6, 2, 3]);
        println!("{}", [8, 7, 5, 1, 9, 4, 6, 2, 3].len());
        assert_eq!(a, 21);
    }
}