use std::{
    env,
    fs,
    io,
    thread,
};
use std::io::{
    Write,
};
use std::sync::mpsc;

use log::{
    debug,
    info,
    warn,
    error,
};
use clap;

use fadafada::yaml::{
    FromYaml,
    yaml_from_str,
};
use fadafada::control::Controller;
use fadafada::resolver::Resolver;

use fadafada_curl::{
    process_graph,
    Contents,
};


fn main() {
    // Apply logger instance
    env_logger::init();

    // Parse cli args
    let m = clap::App::new("fadafada-curl")
        .version("0.0.0")
        .arg(clap::Arg::with_name("ctrl")
             .long("ctrl")
             .short("c")
             .value_name("YAML configuration file defining the control graph")
             .takes_value(true)
             )
        .arg(clap::Arg::with_name("contents"))
        .get_matches();

    let config_src_path = m.value_of("ctrl").unwrap();
    let config_s = fs::read_to_string(&config_src_path).unwrap();
    let config_y = yaml_from_str(&config_s);
    let mut ctrl = Controller::from_yaml(&config_y, None);

    let resolver_src_path = m.value_of("contents").unwrap();
    let resolver_s = fs::read_to_string(&resolver_src_path).unwrap();
    let resolver_y = yaml_from_str(&resolver_s);
    let resolver = Resolver::from_yaml(&resolver_y, None);

    let graph = ctrl.generate(resolver);

    let (tx, rx) = mpsc::channel();
    thread::spawn(|| {
        let _r = process_graph(graph, tx);
    });

    let mut r: Contents;
    loop {
        match rx.recv().unwrap() {
            Some(v) => {
                if v.ready {
                    r = v;
                    drop(rx);
                    break;
                }
            },
            _ => {},
        };
    }

    io::stdout().write_all(r.data.as_slice());
}
