
/*
Aufgabenstellung:
Realisieren sie das Beispiel DV2_07.C aus der Vorlesung

Beispiel DV2_07.C:
1. Abfrage wie viele Zeilen eingegeben werden sollen
2. Allokierung von Speicher für Zeiger
3. Eingabe von Textstrings (Allokierung übernimmt die Stringfunktion)
4. Vertauschen der zweiten und der dritten Zeile durch umhängen der Zeiger
5. Ausgabe der Textzeilen + gleichzeitige Freigabe des Speichers
6. Freigabe des Speichers für die Textzeilen
*/

/*
Realisierung in Rust:

Beispiel DV2_07.C:
1. Abfrage wie viele Zeilen eingegeben werden sollen
    - genauso wie in C
2. Allokierung von Speicher für Zeiger
    - nicht notwendig, der Vektor wird im Heap alloziiert und verwaltet den Speicher selber
3. Eingabe von Textstrings (Allokierung übernimmt die Stringfunktion)
    - wie in C
4. Vertauschen der zweiten und der dritten Zeile durch umhängen der Zeiger
    - ein Vektor bringt die Funktion .swap() mit
5. Ausgabe der Textzeilen + gleichzeitige Freigabe des Speichers
    - fast wie in C, nur die Freigabe geschieht wenn der String den Scope Verlässt
6. Freigabe des Speichers für die Textzeilen
    - die Freigabe geschieht wenn der Vektor den Scope Verlässt
*/

use std::io::{stdin,stdout,Write};      //Für Ein und Ausgabe von stdin

fn main() {
    //Begrüßung
    println!("Einfaches Textprogramm");
    let mut lines: Vec<String> = Vec::new();;
    let mut line = String::new();
    let mut line_n = 0;

    //Abfrage wie viele Zeilen eingelesen werden sollen
    print!("Wie viele Zeilen sollen eingegeben werden ?");
    let _ = stdout().flush();
    stdin().read_line(&mut line).expect("Es wurde keine korrekte Zeile eingegeben");
    if let Some('\n') = line.chars().next_back() {
        line.pop();
    }
    if let Some('\r') = line.chars().next_back() {
        line.pop();
    }
    line_n = line.parse::<usize>().unwrap();

    //Eingabe der Zeilen
    println!("Anzahl Zeilen: {}",&line_n);
    for i in 0..line_n{
        print!("Gib Zeile {} ein: ",i+1);
        line = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut line).expect("Es wurde keine korrekte Zeile eingegeben");
        if let Some('\n') = line.chars().next_back() {
            line.pop();
        }
        if let Some('\r') = line.chars().next_back() {
            line.pop();
        }
        lines.push(line);
    }

    //Vertauschen von Zeilen 2 und drei wenn die Anzahl der Zeilen > 2 ist
    if (line_n>2){
        lines.swap(1,2);
    }

    //Ausgabe der Zeilen
    for i in lines {
        println!("{}",i);
    }
}
