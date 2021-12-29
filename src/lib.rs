use std::time;
use std::sync::mpsc;
use std::thread::{
    sleep,
};

use log::{
    debug,
    info,
    warn,
    error,
};
use curl::easy::Easy;

use fadafada::control::graph::ControllerGraph;

mod error;

mod validator;

pub struct Contents {
    pub ready: bool,
    pub url: String,
    pub data: Vec<u8>,
}

impl Contents {
    pub fn new() -> Contents {
        Contents {
            ready: false,
            url: "".to_string(),
            data: vec![],
        }
    }
}

//pub fn retrieve(b: &mut Vec<u8>, retrieve_url: &str) {
pub fn retrieve(contents: &mut Contents, retrieve_url: &str) {
    let mut curl_easy = Easy::new();
    let mut _rr = curl_easy.url(retrieve_url).unwrap();
    let mut curl_easy_transfer = curl_easy.transfer();
    curl_easy_transfer.write_function(|data| {
        contents.data.extend_from_slice(data);
        contents.url == retrieve_url.to_string();
        contents.ready = true;
        Ok(data.len())
    }).unwrap();
    match curl_easy_transfer.perform() {
        Err(e) => {
            warn!("could not retrieve {}", retrieve_url);
        },
        _ => {},
    };
}

pub fn process_graph(graph: ControllerGraph, tx: mpsc::Sender<Option<Contents>>) -> Result<Contents, error::NoContentError> {
    let mut have_err = false;

    graph.for_each(|v| {
        if have_err {
            return;
        }
        debug!("processing graph entry {:?} delay {:?}", v.1, v.0);
        let req_delay = time::Duration::from_millis(v.0);
        sleep(req_delay);
        
        match tx.send(None) {
            Err(_e) => {
                have_err = true;
                info!("termination detected");
                return;
            },
            _ => {},
        }

        let mut contents = Contents::new();
        retrieve(&mut contents, &v.1);
        match tx.send(Some(contents)) {
            Err(e) => {
                debug!("send error: {:?}", e);
                have_err = true;
            },
            _ => {},
        }
    });
    Err(error::NoContentError{})
}
