use std::env;

mod app;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("You must pass a file name");
        return;
    }

    let mut app = app::App::new(args);

    app.run();
}
