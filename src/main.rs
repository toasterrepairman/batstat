use gtk::prelude::*;

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let glade_src = include_str!("batstat.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    let window: gtk::Window = builder.object("mainwindow").unwrap();

    // Present window to the user
    window.present();
}
