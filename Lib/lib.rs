use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {  //Nachrichten-Enum
    NewJob(Job),
    Terminate,
}

struct Worker {                             //Workerstruct
    id: usize,                              //ID des Workers
    thread: Option<thread::JoinHandle<()>>, //Thread des Workers
}

pub struct ThreadPool{              //Thread Pool
    workers: Vec<Worker>,           //Worker innerhalb des Pools
    sender: mpsc::Sender<Message>,  //Sender um Nachrichten zum Worker zu senden
}

trait FnBox {
    fn call_box(self: Box<Self>);   //Hat irgendwas mit dem Ausführen der Funktion zu tun
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {     //Hat etwas mit dem ausführen der Funktion zu tun
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>; //Job Datentyp um Funktionen auszuführen

impl ThreadPool {
   pub fn new(size: usize) -> ThreadPool {                          //Erstellt den Threadpool
        assert!(size > 0);                                          //Panic wenn die Größe nicht größer als 0 ist
        let mut workers = Vec::with_capacity(size);                 //Erstelle Workerthreads 
        let (sender, receiver) = mpsc::channel();                   //Erstelle den Channel über den die Jobs an die Worker verteilt werden und die Threads terminiert werden können
        let receiver = Arc::new(Mutex::new(receiver));              //Schütze den Receiver mit einem Arc (notwendig damit nicht mehrere Threads auf den Receiver zugreifen)
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));   //Erstelle Worker
        }
        ThreadPool {                                                //Erstelle den Threadpool mit den Workern und einem Sender
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)                              //Führt eine Funktion aus
        where F: FnOnce() + Send + 'static {                    
            let job = Box::new(f);                              //Alloziiere Speicher für die Funktion
            self.sender.send(Message::NewJob(job)).unwrap();    //Sende die Funktion zum Worker
    }

}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker { //Erstellt einen Worker
        let thread = thread::spawn(move ||{ //Erstellt einen Thread
            loop {
                let message = receiver.lock().unwrap().recv().unwrap(); //Versuche eine Nachricht zu empfangen
                match message {                                         //Werte die Nachricht aus
                    Message::NewJob(job) => {                           //Wenn die Nachricht "neuer Job" lautet:
                        job.call_box();                                 //Rufe die Funktion die der Job enthält
                    },
                    Message::Terminate => {                             //Wenn die Nachricht "Terminate" lautet
                        break;                                          //beende den Thread
                    },
                }
            }
        });
        Worker {    //Erstelle den Worker
            id,
            thread: Some(thread),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) { 
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}