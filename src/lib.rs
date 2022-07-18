//! This crate provides a CLI interface to the [fadfada](fadfada) HTTP request multiplexer. 
//!
//! The request structure is defined by providing two yaml files, as described in
//! [fadfada::yaml](fadfada::yaml).
//!
//! On successful retrieval, the data retrieved will be written to standard output.
//!
//! # Content validation
//!
//! `fadfada_curl` optionally performs validation of the content. Currently only a `sha256`
//! validator is implemented, and it will verify that the content reference (in hex) for
//! the data is the sha256 sum of the content retrieved. This feature is only available if built
//! with the `sha256` feature, and the command is run with the `--with-validator=sha256` option.
//!
//! If no validator is specified, the default "noop" validator will be used, which performs no
//! content checking.
use std::{
    time,
    thread,
};
use std::thread::{
    sleep,
};
use std::sync::mpsc;

use log::{
    debug,
    info,
    warn,
};
use curl::easy::Easy;

use fadfada::control::graph::ControllerGraph;
use fadfada::source::Engine;

mod error;

pub mod validator;

pub struct Contents {
    pub ready: bool,
    pub url: String,
    pub engine: Engine,
    pub data: Vec<u8>,
}

impl Contents {
    pub fn new() -> Contents {
        Contents {
            ready: false,
            url: "".to_string(),
            engine: "".to_string(),
            data: vec!(),
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
        contents.url = retrieve_url.to_string();
        contents.ready = true;
        Ok(data.len())
    }).unwrap();
    match curl_easy_transfer.perform() {
        Err(_e) => {
            warn!("could not retrieve {}", retrieve_url);
        },
        _ => {},
    };
}

pub fn process_graph(graph: ControllerGraph, tx: mpsc::Sender<Contents>) {
    let mut have_err = false;


    graph.for_each(|v| {
        if have_err {
            return;
        }
        let req_delay = time::Duration::from_millis(v.0);
        sleep(req_delay);
        debug!("processing graph entry {:?} delay {:?}", v.1, v.0);
  
        let no_content = Contents::new();
        match tx.send(no_content) {
            Err(_e) => {
                have_err = true;
                info!("termination detected");
                return;
            },
            _ => {},
        }

        let tx_sender = tx.clone();
        let _r = thread::spawn(move || {
            let mut contents = Contents::new();
            retrieve(&mut contents, &v.1);
            contents.engine = v.2;
            match tx_sender.send(contents) {
                Err(e) => {
                    debug!("send error: {:?}", e);
                },
                _ => {},
            }
        });
    });
}
