/*
Aufgabenstellung:
Verwaltung eines CD-Archivs
Eine CD-Sammlung soll mit Hilfe einer Datei verwaltet werden.
Für jede CD ist ein Satz in der Verwaltungsdatei vorgesehen.

Menü:
- Datei öffnen wird immer zu Beginn aufgerufen.
- Eingabe und Ausgabe können durch Eingabe von E und A unabhängig voneinander aufgerufen werden.
- Eingabe und Ausgabe können solange wiederholt aufgerufen werden, bis * für Abbruch anstelle E und A eingegeben wird
- Falls der Bediener anstelle *, E, A eine falsche Eingabe tätigt, wird er auf seinen Fehler hingewiesen und die Eingabe wiederholt.


Unterprogramme:
1. Datei öffnen
2. Eingabe neuer Satz
3. Ausgabe aller Sätze
4. Ändern eines Satzes


//Statemachine:

States:
0 - Startzustand, Lade die Datenbank und gehe über in den Menuzustand
1 - Menuzustand, Warte auf Eingabe vom Nutzer und Wechsle abhängig von Eingabe in 2(Eingabe),3(Ausgabe),4(Änderung),5(Ende)
2 - Eingabe, Fordere den Benutzer auf eine neue Eingabe zu tätigen. Notwendige Angaben: Interpret, Titel, Nr wird von System Vergeben, Anzahl an Einträgen wird um eins erhöht, Einträge werden in Datei gespeichert Wechsle danach in Zustand 1
3 - Ausgabe, gib alle enthaltenen Karteikarten aus, wechsle danach in Zustand 1
4 - Änderung, Fordere den Nutzer auf einen Index einzugeben, überprüfe ob Index> größe der DB, Fordere den Nutzer auf Interpret+Titel anzugeben., wechsle danach in den Zustand 1
5 - Ende, Schließe die geöffnete Datei, gib alle alloziierten zeiger frei
*/

use std::fs::File;
use std::process::{Command, Stdio};

//Datensatz
struct Record{
    a_number: i32,
    interpret: [char;41],
    title: [char;41],
}

//Zustandsautomat:

//Zustände
enum States {
    SStart,
    SMenu,
    SIn,
    SOut,
    SMod,
    SFin,
}

//Zustandsaufrufe

//Startzustand, Lade die Datenbank und gehe über in den Menuzustand 
fn fsm_sstart(current_state:&mut States){
    println!("Start");
    println!("Lade Datenbank");    
    let mut file = File::open("Datenbank.db");
    if file.is_err() {
        println!("Fehler - Datei nicht vorhanden, erstelle Datei");
        file = File::create("Datenbank.db");
        file = File::open("Datenbank.db");
    }
    else {
        println!("Datei vorhanden");        
    }
    *current_state = States::SMenu;
}
//Menuzustand, Warte auf Eingabe vom Nutzer und Wechsle abhängig von Eingabe in 2(Eingabe),3(Ausgabe),4(Änderung),5(Ende)
fn fsm_smenu(current_state:&mut States){
    let cls = std::process::Command::new("cls").status();
    println!("{:?}",cls);

    println!("Menu");
    *current_state = States::SIn;
}
//Eingabe, Fordere den Benutzer auf eine neue Eingabe zu tätigen. Notwendige Angaben: Interpret, Titel, Nr wird von System Vergeben, 
//Anzahl an Einträgen wird um eins erhöht, Einträge werden in Datei gespeichert Wechsle danach in Zustand 1
fn fsm_sin(current_state:&mut States){
    println!("Eingabe");
    *current_state = States::SOut;
}
//Ausgabe, gib alle enthaltenen Karteikarten aus, wechsle danach in Zustand 1
fn fsm_sout(current_state:&mut States){
    println!("Ausgabe");
    *current_state = States::SMod;
}
//Änderung, Fordere den Nutzer auf einen Index einzugeben, überprüfe ob Index> größe der DB, Fordere den Nutzer auf Interpret+Titel anzugeben., wechsle danach in den Zustand 1
fn fsm_smod(current_state:&mut States){
    println!("Änderung");
    *current_state = States::SFin;
}
//Ende,
fn fsm_sfin(){
    println!("Ende"); 
}

fn main() {
    let mut current_state: States = States::SStart;
    loop{
        match current_state {
            States::SStart => { 
                fsm_sstart(&mut current_state);
            }
            States::SMenu => {                
                fsm_smenu(&mut current_state);
            }
            States::SIn => {                
                fsm_sin(&mut current_state);
            }
            States::SOut => {                
                fsm_sout(&mut current_state);
            }
            States::SMod => {                
                fsm_smod(&mut current_state);
            }
            States::SFin => {
                fsm_sfin();       
                break;
            }
        }
    }
}
