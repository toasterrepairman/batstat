use gtk::prelude::*;
use battery::*;

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

    let manager = battery::Manager::new().expect("Batteries not found");

    // Battery display
    for (idx, maybe_battery) in manager
        .batteries()
        .expect("Could not list batteries")
        .enumerate() {
        let battery = maybe_battery.expect("Could not initialize battery info");
        let battery_label = gtk::Label::new(Some(&battery.state().to_string()));

        
        window.add(&battery_label);
    }

    
    // Present window to the user
    window.present();
    // Begin the GTK process
    gtk::main();
}
