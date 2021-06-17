use std::collections::HashMap;

use nannou::app::App;
use crate::wgpu;
use crate::Model;
use crate::models::win_status::WinStatus;


fn load_img(app: &App, file_name: &str) -> wgpu::Texture {
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("img").join(file_name);
    return wgpu::Texture::from_path(app, img_path).unwrap();
}

pub fn load_imgs(
    app: &App,
    file_infos: Vec<[String; 2]>
) -> HashMap<String, wgpu::Texture> {
    let mut res: HashMap<String, wgpu::Texture> = HashMap::new();
    for file_info in file_infos.iter() {
        res.insert(file_info[0].clone(), load_img(app, &file_info[1]));
    }

    return res
}

pub fn is_high_score() -> bool {
    // あればファイル内を解析して、上位10位に入っているかで判断。
    // cf: https://doc.rust-jp.rs/rust-by-example-ja/std_misc/fs.html
    return true;
}

pub fn set_initial_state(model: &mut Model) {
    model.win_status = WinStatus::Normal;
    model.ball.set_initial_state();
    model.player.set_initial_state();
    model.game_config.set_initial_state();
}
