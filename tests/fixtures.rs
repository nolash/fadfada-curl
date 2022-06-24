use std::{
    path,
    file,
    env,
    fs,
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

        let tmp_basedir = env::temp_dir();
        let resource_basedir = path::Path::new(&tmp_basedir)
            .join("fadfada_curl");

        let resource_path = resource_basedir.as_path();
        let _r = fs::remove_dir_all(&resource_path);

        TestSetup {
            base_path: exec_dir,
        }
    }

    pub fn path(&self) -> &path::Path {
        return self.base_path;
    }
}
