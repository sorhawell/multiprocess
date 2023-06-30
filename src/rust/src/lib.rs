use extendr_api::prelude::*;
use extendr_engine;
use std::thread;
/// Return string `"Hello world!"` to R.
/// @export

#[derive(Clone, Debug)]
pub struct ParRObj(pub Robj);
unsafe impl Send for ParRObj {}
unsafe impl Sync for ParRObj {}

impl From<Robj> for ParRObj {
    fn from(robj: Robj) -> Self {
        ParRObj(robj)
    }
}
impl std::fmt::Display for ParRObj {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[extendr]
fn run_in_new_process(s: String) -> &'static str {
    let t1 = thread::spawn(move || {
        extendr_engine::start_r_twice(); //like start_r() without once proctection
        let r_home = std::env::var("R_HOME").unwrap();
        println!("R_HOME {r_home}"); //not rprintln
        extendr_api::eval_string(s.as_str())
            .map(ParRObj)
            .map_err(|err| format!("extendr err {}", err))
    });

    rprintln!("waiting for new R process");
    let return_value = t1.join();

    rprintln!("the return from new R process was {:?}", return_value);

    "Hello world!"
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    fn run_in_new_process;
}
