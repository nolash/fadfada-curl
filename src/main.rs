use log::{
    debug,
    warn,
    error,
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

use fadfada::yaml::{
    FromYaml,
    yaml_from_str,
};
use fadfada::control::Controller;
use fadfada::resolver::Resolver;
use fadfada::source::Engine;

use fadfada_curl::{
    process_graph,
    Contents,
};
use fadfada_curl::validator::ValidatorCollection;


fn register_noop<'a>(validators: &mut ValidatorCollection) {
    let engine = "noop".to_string();
    use fadfada::validator::NoopValidator;
    validators.insert(engine, Box::new(NoopValidator{}));
}

fn register_plugins<'a>(validators: &mut ValidatorCollection, plugins: Vec<&str>) {
    if plugins.len() == 0 {
        warn!("no validator plugins detected, falling back on noop - content will NOT be checked");
        register_noop(validators);
        return;
    }
    for v in plugins.iter() {
        let mut r: bool = false;
        match *v {
            "sha256" => {
                #[cfg(feature = "sha256")]
                {
                    let engine = v.to_string();
                    use fadfada::web2::Sha256ImmutableValidator;
                    validators.insert(engine, Box::new(Sha256ImmutableValidator{}));
                    r = true;
                }
                if !r {
                    panic!("Unknown plugin {}", v);
                }
            },
            "noop" => {
                register_noop(validators);
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
    let m = clap::App::new("fadfada-curl")
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

    let graph = ctrl.generate(&resolver);
    let graph_len = graph.len();
    debug!("have {} items in graph", graph_len);

    let (tx, rx) = mpsc::channel();
    thread::spawn(|| {
        let _r = process_graph(graph, tx);
    });

    let mut r: Contents = Contents::new();
    let mut i: usize = 0;
    let mut active = true;
    loop {
        if !active {
            break;
        }
        let v = match rx.recv() { 
            Ok(v) => {
                if v.engine == "" {
                    continue;
                }
                v
            },
            Err(e) => {
                error!("receive error from threads {:?}", e); 
                if i == graph_len {
                    active = false;
                }
                continue;
            }
        };
        i += 1;
        if v.ready {
            i += 1;
            debug!("checking data from url {} with engine {}", v.url, v.engine);
            match resolver.pointer_for(&v.engine) {
                Ok(pointer) => {
                    if validators.verify_by_pointer(&v.engine, &pointer, &v.data) {
                        r = v;
                        drop(rx);
                        break;
                    }
                },
                _ => {
                    continue;
                },
            }
            error!("Invalid content for url {} engine {}", v.url, v.engine);
        }
    }

    let _r = io::stdout().write_all(r.data.as_slice());
}
