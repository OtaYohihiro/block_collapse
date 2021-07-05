use crate::models::ticker::TickerObject;

#[derive(Clone, Copy)]
pub enum Rarity {
    Pratinum,
    Gold,
    Silver,
    Bronze
}

#[derive(Clone)]
pub struct Achievement {
    pub limit: usize,
    pub category: String, // scoreなのか、reflect_cntなのか
    pub title: String,
    pub rarity: Rarity,
    pub description: String,
    pub achieved_at: i64, // unixtimeで保存する
    pub achieved_app_time: f32,
    pub notified: bool,
}

pub const ACHIEVEMENTS: [(usize, &str, &str, Rarity, &str); 8]= [
    (10, "reflect_cnt", "reflect beginner", Rarity::Bronze, ""),
    (50, "reflect_cnt", "reflect intermediate", Rarity::Silver, ""),
    (100, "reflect_cnt", "reflect expert", Rarity::Gold, ""),
    (500, "reflect_cnt", "reflect master", Rarity::Pratinum, ""),
    (50_000, "score", "block beginner", Rarity::Bronze, ""),
    (100_000, "score", "block intermediate", Rarity::Silver, ""),
    (500_000, "score", "block expert", Rarity::Gold, ""),
    (1_000_000, "score", "block master", Rarity::Pratinum, ""),
];

impl Achievement {
    pub fn new(
      limit: usize,
      category: String,
      title: String,
      rarity: Rarity,
      description: String,
      achieved_at: i64,
      achieved_app_time: f32,
      notified: bool,
      // ticker: &mut Ticker,
    ) -> Achievement
    {
        Achievement {
          limit, category, title, rarity, description, achieved_at,
          achieved_app_time, notified
        }
        // ticker.add_observer(achievement);
    }

    pub fn update(&self, t_ojt: &TickerObject) -> bool {
        if self.category == t_ojt.category
            && t_ojt.value >= self.limit && !self.notified {
            let rarity: &str = match self.rarity {
                Rarity::Pratinum => "premium",
                Rarity::Gold => "gold",
                Rarity::Silver => "silver",
                Rarity::Bronze => "bronze",
            };
            println!(
                "{} {} {} // ACHIEVED!!",
                rarity,
                self.title,
                self.description
            );
            return true
        }

        return false
    }
}
