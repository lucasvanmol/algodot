use algonaut::algod::{v2::Algod, AlgodBuilder};
use gdnative::prelude::*;
use gdnative::tasks::{Async, AsyncMethod, Spawner};

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(register_methods)]
pub struct Algodot {
    #[property]
    url: String,

    #[property]
    token: String,
}

impl Algodot {
    fn new(_owner: &Node) -> Self {
        Algodot {
            url: String::new(),
            token: String::new(),
        }
    }
}

struct HealthFn;

impl AsyncMethod<Algodot> for HealthFn {
    fn spawn_with(&self, spawner: Spawner<'_, Algodot>) {
        spawner.spawn(|_ctx, this, mut _args| {
            let algod = this.map(|algodot, _node| algodot.get_algod()).unwrap();

            async move {
                let params = algod.health().await;

                match params {
                    Ok(_) => godot_print!("Health OK!"),
                    Err(err) => godot_error!("{:?}", err),
                }

                ().to_variant()
            }
        });
    }
}

#[methods]
impl Algodot {
    fn get_algod(&self) -> Algod {
        AlgodBuilder::new()
            .bind(&self.url)
            .auth(&self.token)
            .build_v2()
            .unwrap()
    }

    #[export]
    fn get_data(&self, _owner: &Node) -> String {
        "Hello, World!".to_string()
    }
}

fn register_methods(builder: &ClassBuilder<Algodot>) {
    builder.build_method("health", Async::new(HealthFn)).done();
}
