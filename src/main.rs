use gtk::prelude::*;

fn main() {
    build_ui();
}

fn build_ui() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("batstat.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.get_object("mainwindow").unwrap();

    // End program when closed 
    window.connect_destroy( |_| { 
        gtk::main_quit();
    });
    // Present window to the user
    window.present();
    // Begin the GTK process
    gtk::main();
}
