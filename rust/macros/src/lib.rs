pub use paste::paste;

#[macro_export]
macro_rules! asyncmethods {
    ($algod:ident, $node:ident, $this:ident, $( fn $fn:ident($ctx:ident, $args:ident)$( -> $signal:tt)? $block:block); *) => {
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

                    $(

                        builder.add_signal(Signal {
                            name: $signal,
                            args: &[SignalArgument {
                                name: "response",
                                default: ().to_variant(),
                                export_info: ExportInfo::new(VariantType::Dictionary),
                                usage: PropertyUsage::DEFAULT,
                            }],
                        });
                    ) ?
                ) *
            }
        }
    };
}

#[macro_export]
macro_rules! godot_result {
    ($res:ident) => {
        match $res {
            Ok(ok) => godot_print!("{:?}", ok),
            Err(err) => godot_error!("{:?}", err),
        }
    };
}

#[macro_export]
macro_rules! godot_unwrap {
    ($res:ident => $block:block) => {
        match $res {
            Ok($res) => $block,
            Err(err) => godot_error!("{:?}", err),
        }
    };
}
