#[macro_export]
macro_rules! asyncmethods {
    ($algod:ident, $( fn $fn:ident() $block:block), *) => {
        $(
            #[allow(non_camel_case_types)]
            struct $fn;

            impl AsyncMethod<Algodot> for $fn {
                fn spawn_with(&self, spawner: Spawner<'_, Algodot>) {
                    spawner.spawn(|_ctx, this, mut _args| {
                        let $algod = this.map(|algodot, _node| {
                            Rc::clone(&algodot.algod)
                        }).unwrap();

                        async move {
                            $block
                        }
                    });
                }
            }
        ) *

        fn register_methods(builder: &ClassBuilder<Algodot>) {
            $ (
                builder.build_method(stringify!($fn), Async::new($fn)).done();
            ) *
        }
    };
}