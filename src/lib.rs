pub mod y21;

pub trait Solution {
    fn part_1(&self, input_file: &str) -> String;
    fn part_2(&self, input_file: &str) -> String;
}

#[cfg(test)]
mod test_util {
    macro_rules! get_test_file {
        ($file: expr) => {{
            let mut path = std::path::PathBuf::from(env!["CARGO_MANIFEST_DIR"]);
            path.push("resources");
            module_path!()
                .split("::")
                .skip(1)
                .for_each(|p| path.push(p));
            path.push($file);
            path.to_str().unwrap().to_owned()
        }};
    }
    pub(crate) use get_test_file;
}
