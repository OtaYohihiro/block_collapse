use std::io::Read;
use std::fs::File;
use nannou::geom::rect::Rect;
use nannou::geom::vector::vec2;
use nannou::app::DrawScalar;

use crate::models::block::{ Block, BLOCK_SIZE};

// stageを描画する
pub fn execute(stage: usize, win: &Rect<DrawScalar>) -> Vec<Block> {
    let stage_data = load_stage_data(stage);
    let mut blocks: Vec<Block> = vec![];
    let padding = 60.0;

    for (r_idx, row) in stage_data.iter().enumerate() {
        for (idx, i) in row.iter().enumerate() {
            if *i > 0 {
                let b = Block::new(
                    vec2(
                        win.left() + padding + idx as f32 * 30.0,
                        win.top() - padding * 2.0 - r_idx as f32 * 30.0,
                    ),
                    BLOCK_SIZE,
                    *i as u8
                );
                blocks.push(b)
            }
        }
    }

    blocks
}


fn load_stage_data(stage: usize) -> Vec<Vec<usize>> {
    let load_path = format!(
        "/Users/ota/project/rust/block_collapse/stages/{}.txt",
        stage
    );
    let mut file = File::open(load_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut results: Vec<Vec<usize>> = vec![];

    for line in contents.split("\n") {
        let res: Vec<usize> = line.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap()).collect();
        results.push(res);
    }

    results
}
