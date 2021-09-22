use crate::Model;
use crate::models::effect::Effect;

pub const FRAME_RATE: f32 = 60.0;

pub fn add_effect(model: &mut Model, effect: Effect) {
    model.effect_vec.push(effect);
}

pub fn update_effect(model: &mut Model) {
    let mut deletable_effect_idx = vec![];
    for (idx, effect) in model.effect_vec.iter_mut().enumerate() {
        effect.elapsed_time += 1.0 / FRAME_RATE;
        if effect.time <= effect.elapsed_time {
            deletable_effect_idx.push(idx)
        }
    }
    for i in deletable_effect_idx.iter().rev() {
        model.effect_vec.remove(*i);
    }
}
