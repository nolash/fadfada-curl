use std::sync::mpsc;

use log::{
    debug,
    info,
};

use curl::easy::Easy;

use fadafada::control::graph::ControllerGraph;

mod error;


pub fn retrieve(b: &mut Vec<u8>, retrieve_url: &str) {
    let mut curl_easy = Easy::new();
    let mut _rr = curl_easy.url(retrieve_url).unwrap();
    let mut curl_easy_transfer = curl_easy.transfer();
    curl_easy_transfer.write_function(|data| {
        b.extend_from_slice(data);
        Ok(data.len())
    }).unwrap();
    _rr = curl_easy_transfer.perform().unwrap();
}

pub fn process_graph(graph: ControllerGraph, tx: mpsc::Sender<()>) -> Result<Vec<u8>, error::NoContentError> {
    let mut have_err = false;
    graph.for_each(|v| {
        if have_err {
            return;
        }
        debug!("processing graph entry {:?} {:?}", v.0, v.1);
        match tx.send(()) {
            Err(_e) => {
                have_err = true;
                info!("termination detected");
                return;
            },
            _ => {},
        }

        let mut curl_easy = Easy::new();
        let mut _rr = curl_easy.url(&v.1);
    });
    Err(error::NoContentError{})
}
