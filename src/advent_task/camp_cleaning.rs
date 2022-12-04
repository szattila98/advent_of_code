use macros::include_str_arr;

use super::AdventTask;

pub struct CampCleaning;

impl AdventTask for CampCleaning {
    type Solution = u16;

    fn get_name(&self) -> &str {
        "Camp Cleaning"
    }

    fn get_inputs(&self) -> &[Option<&'static str>] {
        include_str_arr!("./inputs/camp_cleaning.txt")
    }

    fn solve_first_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        let mut overlaps = 0;
        for pair in input.iter().flatten() {
            let (x1, x2, y1, y2) = Self::parse_assignment_pair(pair);
            if (x1 <= y1 && x2 >= y2) || (y1 <= x1 && y2 >= x2) {
                overlaps += 1;
            }
        }
        overlaps
    }

    fn solve_second_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        let mut overlaps = 0;
        for pair in input.iter().flatten() {
            let (x1, x2, y1, y2) = Self::parse_assignment_pair(pair);
            if x1 <= y2 && y1 <= x2 {
                overlaps += 1;
            }
        }
        overlaps
    }
}

impl CampCleaning {
    fn parse_assignment_pair(
        pair: &'static str,
    ) -> (
        <CampCleaning as AdventTask>::Solution,
        <CampCleaning as AdventTask>::Solution,
        <CampCleaning as AdventTask>::Solution,
        <CampCleaning as AdventTask>::Solution,
    ) {
        let mut splitted = pair.split(',');
        let first = splitted.next().unwrap();
        let second = splitted.next().unwrap();

        let mut splitted_first = first.split('-');
        let mut splitted_second = second.split('-');
        let x1 = splitted_first.next().unwrap().parse().unwrap();
        let x2 = splitted_first.next().unwrap().parse().unwrap();
        let y1 = splitted_second.next().unwrap().parse().unwrap();
        let y2 = splitted_second.next().unwrap().parse().unwrap();

        (x1, x2, y1, y2)
    }
}