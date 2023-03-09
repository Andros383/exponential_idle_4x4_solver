use crate::BOARD_SIZE;
#[derive(Debug)]
pub struct State {
    pub board: [[i32; BOARD_SIZE]; BOARD_SIZE],
    pub heuristic: i32, //(Distance)
    pub path: Vec<i32>,
}

//Mindent csak átírtam, hogy CSAK a heuristic alapján rendezzen

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heuristic.cmp(&other.heuristic)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.heuristic.partial_cmp(&other.heuristic)
    }
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic == other.heuristic
    }
}
impl Eq for State {
    fn assert_receiver_is_total_eq(&self) {} //Gondolom magának kitalálja?
}
