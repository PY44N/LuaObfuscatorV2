extern crate lua;

use lua::state::State;

fn main() {
    let mut state = State::new();
    let result = state.load_file("_testdata/simple.lua");
    match result {
        Ok(proto) => {
            println!("{:?}", proto)
        }
        Err(e) => println!("Error=>{:?}", e),
    };
}
