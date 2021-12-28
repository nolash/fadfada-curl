use std::{
    path,
    fs,
    thread,
    env,
};
use std::sync::mpsc;

use log::debug;
use url::Url;

use fadafada::control::Controller;
use fadafada::resolver::Resolver;
use fadafada::yaml::{
    FromYaml,
    yaml_from_str,
};
use fadafada_curl::process_graph;

mod fixtures;


#[test]
fn test_graph_yaml() {
    let _ = env_logger::builder().is_test(true).try_init();

    let t = fixtures::TestSetup::new();

    let mut yaml_src_path = path::Path::new(t.path())
        .join("testdata")
        .join("query.yaml");

    let mut s = fs::read_to_string(&yaml_src_path).unwrap();
    let mut y = yaml_from_str(&s);
    let mut ctrl = Controller::from_yaml(&y, None);

    yaml_src_path = path::Path::new(t.path())
        .join("testdata")
        .join("contents.yaml");
    s = fs::read_to_string(&yaml_src_path).unwrap();
    y = yaml_from_str(&s);

    let resolver = Resolver::from_yaml(&y, None);
    let mut graph = ctrl.generate(resolver);
    let r = graph.next().unwrap();
    assert_eq!(0, r.0);
    assert_eq!("file:///tmp/fadafada_curl/a/deadbeef", r.1);
}

#[test]
fn test_graph_processor_nocontent() {
    let _ = env_logger::builder().is_test(true).try_init();

    let t = fixtures::TestSetup::new();
 
    let mut yaml_src_path = path::Path::new(t.path())
        .join("testdata")
        .join("query.yaml");

    let mut s = fs::read_to_string(&yaml_src_path).unwrap();
    let mut y = yaml_from_str(&s);
    let mut ctrl = Controller::from_yaml(&y, None);

    yaml_src_path = path::Path::new(t.path())
        .join("testdata")
        .join("contents.yaml");
    s = fs::read_to_string(&yaml_src_path).unwrap();
    y = yaml_from_str(&s);

    let resolver = Resolver::from_yaml(&y, None);
    let graph = ctrl.generate(resolver);

    let (tx, _rx) = mpsc::channel();
    drop(_rx);
    match process_graph(graph, tx) {
        Ok(_) => {
            panic!("expected error");
        },
        _ => {},
    }
}

#[test]
fn test_graph_processor_content_first() {
    let _ = env_logger::builder().is_test(true).try_init();

    let t = fixtures::TestSetup::new();

    let tmp_basedir = env::temp_dir();
    let resource_basedir = path::Path::new(&tmp_basedir)
        .join("fadafada_curl/a/");
    fs::create_dir_all(&resource_basedir).unwrap();

    let file_foo_path = resource_basedir
        .join("deadbeef");
    let foo_content = b"012345678";

    if file_foo_path.exists() {
        panic!("resource path {:?} already exists, please remove it manually", file_foo_path);
    }
    let mut _r = fs::write(&file_foo_path, &foo_content);
    let file_foo_url = Url::from_file_path(&file_foo_path).unwrap();

    let mut yaml_src_path = path::Path::new(t.path())
        .join("testdata")
        .join("query.yaml");

    let mut s = fs::read_to_string(&yaml_src_path).unwrap();
    let mut y = yaml_from_str(&s);
    let mut ctrl = Controller::from_yaml(&y, None);

    yaml_src_path = path::Path::new(t.path())
        .join("testdata")
        .join("contents.yaml");
    s = fs::read_to_string(&yaml_src_path).unwrap();
    y = yaml_from_str(&s);

    let resolver = Resolver::from_yaml(&y, None);
    let graph = ctrl.generate(resolver);

    let (tx, rx) = mpsc::channel();
    let thr_foo = thread::spawn(|| {
        process_graph(graph, tx);
    });

    let mut r: Vec<u8>;
    loop {
        r = rx.recv().unwrap();
        if r.len() > 0 {
            drop(rx);
            break;
        }
    }
    thr_foo.join();

    assert_eq!(r, b"012345678");
}


#[test]
fn test_graph_processor_content_second() {
    let _ = env_logger::builder().is_test(true).try_init();

    let t = fixtures::TestSetup::new();

    let tmp_basedir = env::temp_dir();
    let resource_basedir = path::Path::new(&tmp_basedir)
        .join("fadafada_curl/b/");

    fs::create_dir_all(&resource_basedir).unwrap();

    let file_foo_path = resource_basedir
        .join("beeffeed");
    if file_foo_path.exists() {
        panic!("resource path {:?} already exists, please remove it manually", file_foo_path);
    }

    let foo_content = b"123456789";

    let mut _r = fs::write(&file_foo_path, &foo_content);
    let file_foo_url = Url::from_file_path(&file_foo_path).unwrap();

    let mut yaml_src_path = path::Path::new(t.path())
        .join("testdata")
        .join("query.yaml");

    let mut s = fs::read_to_string(&yaml_src_path).unwrap();
    let mut y = yaml_from_str(&s);
    let mut ctrl = Controller::from_yaml(&y, None);

    yaml_src_path = path::Path::new(t.path())
        .join("testdata")
        .join("contents.yaml");
    s = fs::read_to_string(&yaml_src_path).unwrap();
    y = yaml_from_str(&s);

    let resolver = Resolver::from_yaml(&y, None);
    let graph = ctrl.generate(resolver);

    let (tx, rx) = mpsc::channel();
    let thr_foo = thread::spawn(|| {
        process_graph(graph, tx);
    });

    let mut r: Vec<u8>;
    loop {
        r = rx.recv().unwrap();
        if r.len() > 0 {
            drop(rx);
            break;
        }
    }
    thr_foo.join();

    assert_eq!(r, b"123456789");
}
