use macros::include_str_arr;

use super::AdventTask;

const START_MARKER_LEN: usize = 4;
const MESSAGE_MARKER_LEN: usize = 14;

pub struct TuningTrouble;

impl AdventTask for TuningTrouble {
    type Solution = usize;

    fn get_name(&self) -> &str {
        "Tuning Trouble"
    }

    fn get_inputs(&self) -> &[Option<&'static str>] {
        include_str_arr!("./inputs/tuning_trouble.txt")
    }

    fn solve_first_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        let line = input.first().unwrap().unwrap();
        Self::search_marker(line, START_MARKER_LEN)
    }

    fn solve_second_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        let line = input.first().unwrap().unwrap();
        Self::search_marker(line, MESSAGE_MARKER_LEN)
    }
}

impl TuningTrouble {
    fn search_marker(line: &str, packet_len: usize) -> <TuningTrouble as AdventTask>::Solution {
        line.chars()
            .collect::<Vec<_>>()
            .windows(packet_len)
            .position(|packet| {
                !packet
                    .iter()
                    .enumerate()
                    .any(|(i, c)| packet[..i].contains(c))
            })
            .unwrap()
            + packet_len
    }
}
