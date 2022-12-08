use macros::include_str_arr;

use super::AdventTask;

pub struct DeviceCleanup;

impl AdventTask for DeviceCleanup {
    type Solution = u32;

    fn get_name(&self) -> &str {
        "Device Cleanup"
    }

    fn get_inputs(&self) -> &[Option<&'static str>] {
        include_str_arr!("./inputs/device_cleanup.txt")
    }

    fn solve_first_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        let mut sizes = 0;
        for line in input.iter().flatten() {
            match () {
                _ if line.contains("cd") => {
                    let to = line.split(' ').last().unwrap();
                    match to {
                        "/" => {
                            continue;
                        }
                        ".." => {
                            todo!()
                        }
                        _ => {
                            todo!()
                        }
                    }
                }
                _ if line.contains("ls") => continue,
                _ => {
                    let first_part = line.split(' ').next().unwrap();
                    if first_part == "dir" {
                        todo!()
                    } else {
                        todo!()
                    }
                }
            }
        }
        sizes
    }

    fn solve_second_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        2
    }
}

trait Node {
    fn get_name(&self) -> &str;
    fn get_size(&self) -> u32;
    fn get_parent(&self) -> &Option<&Box<&dyn Node>>;
    fn find_child(&self, to: &str) -> Option<&Box<&dyn Node>>;
}
