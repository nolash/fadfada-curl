#[allow(dead_code)]
use log::debug;

use std::{
    path,
    env,
    fs,
};

use tempdir;
use url::Url;
use curl::easy::Easy;
use env_logger;

/// Verify that file scheme resource retrieval works
#[test]
fn test_url_get_file() {
    env_logger::init();

    let tmp_basedir = env::temp_dir();
    let resource_basedir = path::Path::new(&tmp_basedir)
        .join("fadafada_curl");
    fs::create_dir_all(&resource_basedir).unwrap();
    let _t = tempdir::TempDir::new("fadafada_curl/");

    let file_foo_path = resource_basedir
        .join("deadbeef");
    let foo_content = b"012345678";
    let mut _r = fs::write(&file_foo_path, &foo_content);
    let file_foo_url = Url::from_file_path(file_foo_path).unwrap();

    let file_bar_path = resource_basedir
        .join("feedbeef");
    _r = fs::write(&file_bar_path, b"01234578");
    let file_bar_url = Url::from_file_path(file_bar_path).unwrap();

    let mut b = Vec::new();
    {
        let mut curl_easy = Easy::new();
        let mut _rr = curl_easy.url(file_foo_url.as_str()).unwrap();
        let mut curl_easy_transfer = curl_easy.transfer();
        curl_easy_transfer.write_function(|data| {
            b.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        _rr = curl_easy_transfer.perform().unwrap();
    }
    assert_eq!(foo_content, b.as_mut_slice());
}
