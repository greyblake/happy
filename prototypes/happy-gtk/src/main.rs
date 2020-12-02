use gtk::prelude::*;
use gio::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use gtk::{MenuBar, MenuItem, Menu};
use gtk::{Orientation};

fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Happy HTTP");
        window.set_default_size(600, 600);


        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

        let menu_bar = build_menu_bar();
        v_box.pack_start(&menu_bar, false, false, 0);

        let main_view = build_main_view();
        v_box.pack_start(&main_view, false, false, 0);

        window.add(&v_box);

        window.show_all();
    });

    application.run(&[]);
}


fn build_menu_bar() -> MenuBar {
    let menu = Menu::new();
    let menu_bar = MenuBar::new();
    let file = MenuItem::with_label("File");

    let exit = MenuItem::with_label("Exit");
    let about = MenuItem::with_label("About");

    exit.connect_activate( move |_| {
        println!("EXIT!");
    });

    about.connect_activate( move |_| {
        println!("ABOUT!");
    });

    menu.append(&about);
    menu.append(&exit);

    file.set_submenu(Some(&menu));
    menu_bar.append(&file);

    menu_bar
}

fn build_main_view() -> gtk::Box {
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 10);

    let input1 = gtk::Entry::new();
    let input2 = gtk::Entry::new();

    hbox.pack_start(&input1, false, false, 0);
    hbox.pack_start(&input2, false, false, 0);

    hbox
}
