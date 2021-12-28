use std::{
    path,
    fs,
};
use std::sync::mpsc;

use log::debug;

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
