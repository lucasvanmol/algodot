pub use paste::paste;

/// Macro for simplifying AsyncMethod impl
#[macro_export]
macro_rules! asyncmethods {
    ($algod:ident, $node:ident, $this:ident, $( fn $fn:ident($ctx:ident, $args:ident) $block:block) *) => {
        $crate::paste! {
            $(
                #[allow(non_camel_case_types)]
                struct [<__ $fn>];

                impl AsyncMethod<Algodot> for [<__ $fn>] {
                    fn spawn_with(&self, spawner: Spawner<'_, Algodot>) {
                        spawner.spawn(|$ctx, $this, mut $args| {
                            #[allow(unused_variables)]
                            let ($algod, $node) = $this.map(|algodot, node| {
                                (Rc::clone(&algodot.algod), node.claim())
                            }).unwrap();

                            $block

                        });
                    }
                }
            ) *

            fn register_methods(builder: &ClassBuilder<Algodot>) {
                $ (
                    builder.build_method(stringify!($fn), Async::new([<__ $fn >])).done();
                ) *
            }
        }
    };
}

/// Converts from `Result<T, E>` to `Option<T>`, printing the error to godot's stderr.
#[macro_export]
macro_rules! godot_unwrap {
    ($res:ident) => {
        match $res {
            Ok(ok) => Some(ok),
            Err(err) => {
                godot_error!("{:?}", err);
                None
            }
        }
    };
}
