extern crate gtk;

use std::process;
//use std::thread;
use gtk::*;

//use std::sync::Arc;
//use std::sync::atomic::{AtomicUsize, Ordering};

pub mod frontend{
    use gtk::*;

    pub struct App {
        pub window: Window,
        pub header: Header,
        pub content: Content,
    }

    impl App {
        pub fn new() -> App {
            // Erstelle ein neues Fenster
            let window = Window::new(WindowType::Toplevel);
            // Erstelle ein Header-Objekt
            let header = Header::new();
            // Erstelle den Content
            let content = Content::new();


            // Setze die Header Bar als Titelbar
            window.set_titlebar(&header.container);
            // Setze den Titel des Fensters
            window.set_title("Rechner");
            // Setze den Windowmanager
            window.set_wmclass("app-name", "App name");
            // Setze das Icon für das Fenster
            Window::set_default_icon_name("iconname");
            // Setze das Resizaable Flag
            window.set_resizable(false);  
            // Füge den Content hinzu
            window.add(&content.container);
            
            // Programmiere den Exit-Button
            window.connect_delete_event(move |_, _| {
                main_quit();
                Inhibit(false)
            });

            // Erstelle die App
            App { window, header , content}
        }
    }

    pub struct Header {
        pub container: HeaderBar,
    }

    impl Header {
        fn new() -> Header {
            // Erstelle eine Header Bar
            let container = HeaderBar::new();

            // Zeige den Schließen Button
            container.set_show_close_button(true);

            // Returns the header and all of it's state
            Header { container }
        }
    }

    pub struct Content {
        pub container: Box,
        pub grid: Grid,
        pub text_view: TextView,       
    }

    impl Content {
        fn new() -> Content {
            // Erstelle eine vertikale ContentBox
            let container = Box::new(Orientation::Vertical,0);

            // Erstelle den Inhalt
            // Erstelle ein Gitter für die Buttons:
            let grid = Grid::new();

            //Erstelle das Texteingabefenster
            let text_view = TextView::new();

            //Erstelle die Buttons
            let button1 = Button::new_with_mnemonic("7");
            let button2 = Button::new_with_mnemonic("8");
            let button3 = Button::new_with_mnemonic("9");
            let button4 = Button::new_with_mnemonic("/");
            let button5 = Button::new_with_mnemonic("4");
            let button6 = Button::new_with_mnemonic("5");
            let button7 = Button::new_with_mnemonic("6");
            let button8 = Button::new_with_mnemonic("*");
            let button9 = Button::new_with_mnemonic("1");
            let button10 = Button::new_with_mnemonic("2");
            let button11 = Button::new_with_mnemonic("3");
            let button12 = Button::new_with_mnemonic("-");
            let button13 = Button::new_with_mnemonic(",");
            let button14 = Button::new_with_mnemonic("0");
            let button15 = Button::new_with_mnemonic(" ");
            let button16 = Button::new_with_mnemonic("+");

            //Einstellung der Elemente
            text_view.set_editable(false);

            grid.attach(&button1,0,1,1,1);
            grid.attach(&button2,1,1,1,1);
            grid.attach(&button3,2,1,1,1);
            grid.attach(&button4,3,1,1,1);
            grid.attach(&button5,0,2,1,1);
            grid.attach(&button6,1,2,1,1);
            grid.attach(&button7,2,2,1,1);
            grid.attach(&button8,3,2,1,1);
            grid.attach(&button9,0,3,1,1);
            grid.attach(&button10,1,3,1,1);
            grid.attach(&button11,2,3,1,1);
            grid.attach(&button12,3,3,1,1);
            grid.attach(&button13,0,4,1,1);
            grid.attach(&button14,1,4,1,1);
            grid.attach(&button15,2,4,1,1);
            grid.attach(&button16,3,4,1,1);

            container.pack_start(&text_view,false,false,1);
            container.pack_end(&grid,false,false,1);
            Content {container,grid,text_view}
        }
    }
}

fn run(){
    // Initialisiert GTK
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    // Erstellt die App
    let app = frontend::App::new();

    //Zugriff auf Elemente des Frontends
    //app.content.text_view

    // Macht alle Widgeds sichtbar
    app.window.show_all();

    // Startet die GTK Event-Schleife
    gtk::main();
}

fn main() {
    run();
}