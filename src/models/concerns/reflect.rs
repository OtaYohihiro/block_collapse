use nannou::app::App;
use crate::Model;

// pub trait ReflectObject {}

pub trait ReflectLogic {
    // reflectも実装したいのだが、どうしてもballとblockでreflectの
    // 挙動が異なる。のと引数の型も違うのもあったりして無理
    // fn reflect(&mut self, app: &App, target: &impl ReflectObject) -> bool;

    fn reflect_sound(&self, app: &App, model: &mut Model) {
        let assets = app.assets_path().unwrap();
        let path = assets.join("sounds").join("反射音.wav");
        let sound = audrey::open(path).expect("failed to load sound");
        model.stream
            .send(move |audio| { audio.sounds.push(sound) })
            .ok();
    }
}
