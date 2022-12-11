#![allow(dead_code)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod data_structure;
pub mod dp;
pub mod graph;
pub mod math;
pub mod misc;
