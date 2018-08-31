/*
Aufgabenstellung:
Array mit 100 Float Zahlen erstellen.
Arrayzugriff auf 3 Arten:
-Indizes x[i]
-Berechnete Zeiger *(x+i)
-Inkrementierter Zeiger x++

Berechnung von:
-Maximum + Index
-Minimum + Index
-Index des Elementes mit der Größten Abweichung.
-Standartabweichung
*/

use std::rand;
use std::rand::Rng;

fn main() {
    let mut array = [0.0;100];  //Arraydeklaration mit 100 Feldern mit dem Inhalt 0 [Datentyp;Anzahl]
                                //Alternative: Listendeklaration [0,1,2,3,4,5,7...]

    for x in array.iter(){
        print!("{} ", x);
    }
}