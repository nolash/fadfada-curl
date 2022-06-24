use log::debug;

use std::{
    path,
    env,
    fs,
};

use tempdir;
use url::Url;
use env_logger;

use fadfada_curl::{
    retrieve,
    Contents,
};

pub fn retrieve_url(contents: &mut Contents, retrieve_url: url::Url) {
    retrieve(contents, retrieve_url.as_str());
}

/// Verify that file scheme resource retrieval works
#[test]
fn test_url_get_file() {
    env_logger::init();

    let tmp_basedir = env::temp_dir();
    let resource_basedir = path::Path::new(&tmp_basedir)
        .join("fadfada_curl");
    fs::create_dir_all(&resource_basedir).unwrap();
    let _t = tempdir::TempDir::new("fadfada_curl/");

    let file_foo_path = resource_basedir
        .join("deadbeef");
    let foo_content = b"012345678";
    let mut _r = fs::write(&file_foo_path, &foo_content);
    let file_foo_url = Url::from_file_path(file_foo_path).unwrap();

    let file_bar_path = resource_basedir
        .join("feedbeef");
    _r = fs::write(&file_bar_path, b"01234578");
    let _file_bar_url = Url::from_file_path(file_bar_path).unwrap();

    let mut content: Contents = Contents::new();
    retrieve_url(&mut content, file_foo_url);

    assert_eq!(foo_content, content.data.as_mut_slice());
}
