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
}

pub const ACHIEVEMENTS: [(usize, &str, &str, Rarity, &str); 8]= [
    (10, "reflect_cnt", "反射初心者", Rarity::Bronze, ""),
    (50, "reflect_cnt", "反射中級者", Rarity::Silver, ""),
    (100, "reflect_cnt", "反射上級者", Rarity::Gold, ""),
    (500, "reflect_cnt", "反射マスター", Rarity::Pratinum, ""),
    (50_000, "score", "ブロック崩し入門者", Rarity::Bronze, ""),
    (100_000, "score", "ブロック崩し中級者", Rarity::Silver, ""),
    (500_000, "score", "ブロック崩し上級者", Rarity::Gold, ""),
    (1_000_000, "score", "ブロック崩しマスター", Rarity::Pratinum, ""),
];

impl Achievement {
    pub fn new(
      limit: usize,
      category: String,
      title: String,
      rarity: Rarity,
      description: String,
      // ticker: &mut Ticker,
    ) -> Achievement
    {
        Achievement {
          limit, category, title, rarity, description,
        }
        // ticker.add_observer(achievement);
    }

    pub fn update(&self, t_ojt: &TickerObject) -> bool {
        if self.category == t_ojt.category && t_ojt.value >= self.limit {
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
