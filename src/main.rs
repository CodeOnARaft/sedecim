use std::env;

mod app;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        show_help();
        return;
    }

    let mut app = app::App::new(args);

    app.run();
}

fn show_help(){
    println!("\n\n");
    println!("                _              _            ");
    println!(" ___   ___   __| |  ___   ___ (_) _ __ ___  ");
    println!("/ __| / _ \\ / _` | / _ \\ / __|| || '_ ` _ \\ ");
    println!("\\__ \\|  __/| (_| ||  __/| (__ | || | | | | |");
    println!("|___/ \\___| \\__,_| \\___| \\___||_||_| |_| |_|\n\n");

    println!("File Name is Required!\n");
    println!("sedecim <File Name>");
    println!("Example: sedecim research.txt");
    
    
}
