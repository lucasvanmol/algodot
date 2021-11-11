use std::rc::Rc;
use algonaut::algod::{v2::Algod, AlgodBuilder, AlgodCustomEndpointBuilder};
use gdnative::prelude::*;
use gdnative::tasks::{Async, AsyncMethod, Spawner};
use algodot_macros::*;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(register_methods)]
pub struct Algodot {
    #[property]
    url: String,

    #[property]
    token: String,

    #[property]
    headers: Dictionary<Shared>,

    algod: Rc<Algod>,
}

impl Algodot {
    fn new(_owner: &Node) -> Self {
        Algodot {
            url: String::new(),
            token: String::new(),
            headers: Dictionary::<Shared>::new_shared(),

            // algod will be initialised on _enter_tree()
            // leave theses default values here for now
            algod: Rc::new(
                AlgodBuilder::new()
                    .bind("http://localhost:4001")
                    .auth("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
                    .build_v2()
                    .unwrap(),
            ),
        }
    }
}

#[methods]
impl Algodot {
    #[export]
    fn _enter_tree(&mut self, _owner: &Node) {
        let algod: Algod;
        if self.token.is_empty() {
            let h1: Vec<(String, String)> = self
                .headers
                .iter()
                .map(|(key, val)| (key.to_string(), val.to_string()))
                .collect();
            let h2 = h1
                .iter()
                .map(|(key, val)| (key.as_str(), val.as_str()))
                .collect();

            algod = AlgodCustomEndpointBuilder::new()
                .bind(&self.url)
                .headers(h2)
                .build_v2()
                .unwrap();
        } else {
            algod = AlgodBuilder::new()
                .bind(&self.url)
                .auth(&self.token)
                .build_v2()
                .unwrap();
        }
        self.algod = Rc::new(algod);
    }

    #[export]
    fn get_data(&self, _owner: &Node) -> String {
        "Hello, World!".to_string()
    }
}

asyncmethods!(
    algod,
    fn health() {
        let status = algod.health().await;

        match status {
            Ok(_) => godot_print!("Health OK!"),
            Err(err) => godot_error!("{:?}", err),
        }

        ().to_variant()
    },
    fn suggested_transaction_params() {
        let params = algod.suggested_transaction_params().await;

        match params {
            Ok(params) => godot_print!("{:?}", params),
            Err(err) => godot_error!("{:?}", err),
        }

        ().to_variant()
    }
);
