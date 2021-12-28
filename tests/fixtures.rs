use std::{
    path,
    file,
};

pub struct TestSetup {
    base_path: &'static path::Path,
}

impl TestSetup {
    pub fn new() -> TestSetup {
        let exec_dir = path::Path::new(file!())
            .parent()
            .unwrap()
            .parent()
            .unwrap();

        TestSetup {
            //base_path: exec_dir.to_str().unwrap().to_string(),
            base_path: exec_dir,
        }
    }

    pub fn path(&self) -> &path::Path {
        //return path::Path::new(&self.base_path);
        return self.base_path;
    }
}
