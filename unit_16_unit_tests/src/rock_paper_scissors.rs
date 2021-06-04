#[derive(Copy, Clone)]
pub enum RPS {
    Rock,
    Paper,
    Scissor
}

#[derive(PartialEq, Debug)]
pub enum GameResult {
    Win,
    Loss,
    Tied
}

impl RPS {
    pub fn get_result(&self, opponent: RPS) -> GameResult {
        match (*self, opponent) {
            (RPS::Rock, RPS::Scissor) | (RPS::Scissor, RPS::Paper) | (RPS::Paper, RPS::Rock) => GameResult::Win,
            (RPS::Scissor, RPS::Rock) | (RPS::Paper, RPS::Scissor) | (RPS::Rock, RPS::Paper) => GameResult::Loss,
            _ => GameResult::Tied
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{GameResult, RPS};

    #[test]
    fn rock_beats_scissor() {
        // arrange
        let challenger = RPS::Rock;
        let opponent = RPS::Scissor;

        // act
        let result = challenger.get_result(opponent);

        // assert
        assert_eq!(GameResult::Win, result);
    }

    #[test]
    fn paper_beats_rock() {
        // arrange
        let challenger = RPS::Paper;
        let opponent = RPS::Rock;

        // act
        let result = challenger.get_result(opponent);

        // assert
        assert_eq!(GameResult::Win, result);
    }

    #[test]
    fn scissor_beats_paper() {
        // arrange
        let challenger = RPS::Scissor;
        let opponent = RPS::Paper;

        // act
        let result = challenger.get_result(opponent);

        // assert
        assert_eq!(GameResult::Win, result);
    }
}