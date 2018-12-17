extern crate gtk;

use std::process;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use gtk::*;

pub enum Message {
    MPlus,
    MMinus,
    MMulti,
    MDiv,
    MZero,
    MOne,
    MTwo,
    MTree,
    MFour,
    MFive,
    MSix,
    MSeven,
    Meight,
    Mnine,
}

pub struct Frontend {
    pub window: Window,
    pub header: Header,
    pub content: Content,
}

pub struct Header {
    pub container: HeaderBar,
}

pub struct Content {
    pub container: Box,
    pub grid: Grid,
    pub text_view: TextView,
    pub buffer:TextBuffer,
    
}
    
pub struct Backend{
    receiver: Receiver<Message>,
}


//Main
fn main() {
    // Initialisiert GTK
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }
    // Erstellt einen Channel
    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    // Erstellt die App
    let frontend = Frontend::new(tx);
    let backend = Backend::new(rx);

    //Führt das Backend aus
    thread::spawn(move || {backend.run()});
    frontend.window.show_all();

    // Startet die GTK Event-Schleife
    gtk::main();
}

impl Backend {
    pub fn new(receiver: Receiver<Message>) -> Backend{ // Erstellt ein neues Backendobjekt
        Backend {receiver}
    }
    pub fn run(&self){                                  
        loop{
            let received = self.receiver.recv().unwrap();
            match received {
                Message::MPlus   => println!("Plus"),
                Message::MMinus  => println!("Minus"),
                Message::MMulti  => println!("Mal"),
                Message::MDiv    => println!("Geteilt"),
                Message::MZero   => println!("0"),
                Message::MOne    => println!("1"),
                Message::MTwo    => println!("2"),
                Message::MTree   => println!("3"),
                Message::MFour   => println!("4"),
                Message::MFive   => println!("5"),
                Message::MSix    => println!("6"),
                Message::MSeven  => println!("7"),
                Message::Meight  => println!("8"),
                Message::Mnine   => println!("9"),
            }
        }
    }
} 

impl Frontend {
    pub fn new(sender: Sender<Message>) -> Frontend {
        // Erstelle ein neues Fenster
        let window = Window::new(WindowType::Toplevel);
        // Erstelle ein Header-Objekt
        let header = Header::new();
        // Erstelle den Content
        let content = Content::new(sender);

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
        Frontend { window, header , content}
    }
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

impl Content {
    fn new(sender: Sender<Message>) -> Content {
        // Erstelle eine vertikale ContentBox
        let container = Box::new(Orientation::Vertical,0);

        // Erstelle den Inhalt
        // Erstelle ein Gitter für die Buttons:
        let grid = Grid::new();

        //Erstelle das Texteingabefenster
        let text_view = TextView::new();
        //Textview-buffer
        let buffer = text_view.get_buffer().expect("Couldn't get window");

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

        //Setzen der Callbackfunktion für die Buttons
        let butsend = sender.clone();
        button1.connect_clicked(move |_but| { butsend.send(Message::MSeven).unwrap();});
        let butsend = sender.clone();
        button2.connect_clicked(move|_but| { butsend.send(Message::MSeven).unwrap();});
        let butsend = sender.clone();
        button3.connect_clicked(move|_but| {butsend.send(Message::MSeven).unwrap();});
        let butsend = sender.clone();
        button4.connect_clicked(move|_but| {butsend.send(Message::MSeven).unwrap();});
        let butsend = sender.clone();
        button5.connect_clicked(move|_but| {butsend.send(Message::MSeven).unwrap();});
        let butsend = sender.clone();
        button6.connect_clicked(move|_but| {butsend.send(Message::MSeven).unwrap();});
        let butsend = sender.clone();
        button7.connect_clicked(move|_but| {butsend.send(Message::MSeven).unwrap();});
        let butsend = sender.clone();
        button8.connect_clicked(move|_but| {butsend.send(Message::MSeven).unwrap();});
        let butsend = sender.clone();
        button9.connect_clicked(move|_but| {butsend.send(Message::MSeven).unwrap();});
        let butsend = sender.clone();
        button10.connect_clicked(move|_but| {butsend.send(Message::MSeven).unwrap();});
        let butsend = sender.clone();
        button11.connect_clicked(move|_but| {butsend.send(Message::MSeven).unwrap();});
        let  butsend = sender.clone();
        button12.connect_clicked(move|_but| {butsend.send(Message::MSeven).unwrap();});
        let butsend = sender.clone();
        button13.connect_clicked(move|_but| {butsend.send(Message::MSeven).unwrap();});
        let butsend = sender.clone();
        button14.connect_clicked(move|_but| {butsend.send(Message::MSeven).unwrap();});
        let butsend = sender.clone();
        button15.connect_clicked(move|_but| {butsend.send(Message::MSeven).unwrap();});
        let butsend = sender.clone();
        button16.connect_clicked(move|_but| {butsend.send(Message::MSeven).unwrap();});

        //Einstellung der Elemente
        text_view.set_editable(false);
        buffer.set_text("testtext");

        //Hänge alle Buttons an den Grid
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

        //Packe den Grid
        container.pack_start(&text_view,false,false,1);
        container.pack_end(&grid,false,false,1);

        //return
        Content {container,grid,text_view,buffer}
    }
}