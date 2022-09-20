use gdnative::prelude::*;
use tokio::{
    runtime::{Builder, Runtime},
    task::LocalSet,
};

// https://godot-rust.github.io/book/recipes/async-tokio.html
mod algod;

thread_local! {
    static EXECUTOR: &'static SharedLocalPool = {
        Box::leak(Box::new(SharedLocalPool::default()))
    };
}

#[derive(Default)]
struct SharedLocalPool {
    local_set: LocalSet,
}

impl futures::task::LocalSpawn for SharedLocalPool {
    fn spawn_local_obj(
        &self,
        future: futures::task::LocalFutureObj<'static, ()>,
    ) -> Result<(), futures::task::SpawnError> {
        self.local_set.spawn_local(future);

        Ok(())
    }
}

#[derive(NativeClass)]
#[inherit(Node)]
struct AsyncExecutorDriver {
    runtime: Runtime,
}

impl AsyncExecutorDriver {
    fn new(_owner: &Node) -> Self {
        AsyncExecutorDriver {
            runtime: Builder::new_current_thread()
                .enable_io() // optional, depending on your needs
                .enable_time() // optional, depending on your needs
                .build()
                .unwrap(),
        }
    }
}

#[methods]
impl<'a> AsyncExecutorDriver {
    #[method]
    fn _process(&self, _owner: &Node, _delta: f64) {  
        //runs a process function in rust
        EXECUTOR.with(|e| {
            self.runtime
                .block_on(async {
                    e.local_set
                        .run_until(async { tokio::task::spawn_local(async {}).await }) //uses tokio dependencies
                        .await
                })
                .unwrap()
        })
    }
}

fn init(handle: InitHandle) { 
    //binder
    gdnative::tasks::register_runtime(&handle);
    gdnative::tasks::set_executor(EXECUTOR.with(|e| *e));

    handle.add_class::<algod::Algodot>();
    handle.add_class::<AsyncExecutorDriver>();
}

godot_init!(init); //init script for godot
