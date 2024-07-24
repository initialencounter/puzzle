use puzzle_lib::Puzzle;
use napi_derive::napi;

#[napi(js_name = "PuzzleCore")]
pub struct PuzzleCore {
  inner: Puzzle,
}

#[napi]
impl PuzzleCore {
  #[napi(constructor)]
  pub fn new(mode: i32) -> Self {
    PuzzleCore {
      inner: Puzzle::new(mode as usize),
    }
  }

  #[napi]
  pub fn move_sequence(&mut self, sequence: String) -> bool {
    self.inner.move_sequence(&sequence as &str)
  }

  #[napi]
  pub fn get_puzzle(&self) -> Vec<Vec<i32>> {
    self.inner.puzzle.clone()
  }


  #[napi]
  pub fn duration(&self) -> String {
    self.inner.duration()
  }

  #[napi]
  pub fn get_cmds_str(&self) -> String {
    self.inner.cmds_str.clone()
  }

  #[napi]
  pub fn get_mode(&self) -> i32 {
    self.inner.mode as i32
  }
}