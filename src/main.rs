use log::{
    debug,
    warn
};
use std::{
    fs,
    io,
    thread,
};
use std::io::{
    Write,
};
use std::sync::mpsc;

use clap;

use fadafada::yaml::{
    FromYaml,
    yaml_from_str,
};
use fadafada::control::Controller;
use fadafada::resolver::Resolver;
use fadafada::source::Engine;

use fadafada_curl::{
    process_graph,
    Contents,
};
use fadafada_curl::validator::ValidatorCollection;


fn register_plugins<'a>(validators: &mut ValidatorCollection, plugins: Vec<&str>) {
    if plugins.len() == 0 {
        warn!("no validator plugins detected");
        return;
    }
    for v in plugins.iter() {
        match *v {
            "sha256" => {
                #[cfg(feature = "sha256")]
                {
                    let engine = v.to_string();
                    use fadafada::web2::Sha256ImmutableValidator;
                    validators.insert(engine, Box::new(Sha256ImmutableValidator{}));
                }
            },
            _ => {
                panic!("Unknown plugin {}", v);
            },
        };
    }
}


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
        .arg(clap::Arg::with_name("validators")
             .long("with-validator")
             .value_name("Add given validator engine")
             .multiple(true)
             .takes_value(true)
             )
        .arg(clap::Arg::with_name("contents"))
        .get_matches();

    let mut validators = ValidatorCollection::new();
    let mut plugins: Vec<&str> = vec![];
    match m.values_of("validators") {
        Some(plugin_validators) => {
            debug!("have plugin validators {:?}", plugin_validators);
            for plugin in plugin_validators {
                plugins.push(plugin);
            }
        },
        _ => {},
    }
    register_plugins(&mut validators, plugins);

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

    let r: Contents;
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

    let _r = io::stdout().write_all(r.data.as_slice());
}
