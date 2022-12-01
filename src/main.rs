use std::env;

mod app;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = app::App::new();

   let _= app.run(args);
}
