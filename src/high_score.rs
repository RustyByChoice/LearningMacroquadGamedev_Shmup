use std::fs;

const HIGH_SCORE_PATH: &str = "highscore.dat";

pub struct HighScore {
    score : u32,
    high_score : u32,
    high_score_toppled : bool,
}

impl HighScore {
    pub fn new() -> HighScore {
        let score : u32 = 0;
        let high_score : u32 = fs::read_to_string(HIGH_SCORE_PATH)
            .map_or(Ok(0), |i| i.parse::<u32>())
            .unwrap_or(0);
        let high_score_toppled = false;

        HighScore { 
            score: score, 
            high_score: high_score,
            high_score_toppled: high_score_toppled
        }
    }

    pub fn save_high_score(&self) {
        if self.score == self.high_score {
            fs::write(HIGH_SCORE_PATH, self.high_score.to_string()).ok();
        }
    }

    pub fn add(&mut self) {
        self.score += 1;

        if self.high_score < self.score {
            self.high_score_toppled = true;
            self.high_score = self.score;
        }
    }

    pub fn clear(&mut self) {
        self.score = 0;
        self.high_score_toppled = false;
    }

    pub fn get_current_score(&self) -> u32 {
        self.score
    }

    pub fn get_current_high(&self) -> u32 {
        self.high_score
    }

    pub fn is_new_high(&self) -> bool {
        self.high_score_toppled
    }
}