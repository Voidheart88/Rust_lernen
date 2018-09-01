/*
Aufgabenstellung:
Array mit 1000 Float Zahlen erstellen.
Berechnung von:
-Maximum
-Minimum
-Index des Elementes mit der Größten Abweichung vom Durchschnitt
*/

extern crate rand;
extern crate rayon;

use rand::prelude::*;               //Für den Zufallszahlengenerator
use std::time::Instant;             //Für Zeitmessungen
use std::f32;                       //Für minimal und Maximalwerte von Gleitkommazahlen
use rayon::prelude::*;              //Für Datenparallelität

fn main() {
    let mut array : [f32;1000] = [0.0;1000];//Arraydeklaration mit 1000 Feldern [Datentyp;Anzahl] initialisiert mit 0.0
                                            //Alternative: Listendeklaration [0,1,2,3,4,5,7...] (wäre nur etwas anstrengend) 
                                            //TODO herausfinden ob man das Array direkt mit Zufallszahlen initialisieren kann
    let mut rand_max: f32=f32::MIN;         //Enthält den Maximalwert im Array -initialisiert als Minimum von f32
    let mut rand_min: f32=f32::MAX;         //Enthält den Minimalwert im Array -initialisiert als Maximum von f32
    let mut average:  f32=0.0;              //Enthält den Durchschnitt 
    let mut max_dev: f32 = f32::MIN;        //Enthält den Wert der maximalen Abweichung
    let mut index_dev: usize=0;             //Enthält den Index an dem die größte Abweichung ist

    let now = Instant::now();

    //Versuch ohne Parallelbearbeitung:
    println!("Zeit vergangen - Start normal: {:?}",now.elapsed() );

    //Fülle das Array mit Werten
    for i in 0..array.len(){
        array[i] = random::<f32>();         //speichere eine Zufallszahl an die Stelle des Arrays
    }
    println!("Zeit vergangen - nach füllen des Arrays: {:?}",now.elapsed() );

    for i in 0..array.len(){
        if rand_max < array[i] {rand_max = array[i];}
        if rand_min > array[i] {rand_min = array[i];}
        average += array[i];
    }
    average /= 1000.0; //sollte gegen 0.5 gehen

    //Berechnung der maximalen Abweichung
    for i in 0..array.len(){
        let dev = array[i] - average;   //Berechnung der aktuellen differenz
        if dev.abs() > max_dev {        //Wenn die Abweichung größer als die bisherige maximale Abweichung war,
            max_dev = dev.abs();        //aktualisiere die Abweichung
            index_dev = i;              //aktualisiere den Index
        }
    }
    println!("Zeit vergangen - nach Auswertung: {:?} \n",now.elapsed());
    println!("Maximum im Array: {}", rand_max );
    println!("Minimum im Array: {}", rand_min );
    println!("Durchschnitt: {}", average );
    println!("Maximale Abweichung: {}", max_dev);
    println!("Index: {} \n", index_dev);

    //Versuch mit Datenparallelität    
    let mut array2 : [f32;100] = [0.0;100];     //Arraydeklaration mit 100 Feldern [Datentyp;Anzahl] initialisiert mit 0.0
    let mut rand_max2: f32=f32::MIN;            //Enthält den Maximalwert im Array -initialisiert als Minimum von f32
    let mut rand_min2: f32=f32::MAX;            //Enthält den Minimalwert im Array -initialisiert als Maximum von f32
    let mut average2:  f32=0.0;                 //Enthält den Durchschnitt 
    let mut max_dev2: f32 = f32::MIN;           //Enthält den Wert der maximalen Abweichung
    let mut index_dev2: usize=0;                //Enthält den Index an dem die größte Abweichung ist

    let now2 = Instant::now();
    println!("Zeit vergangen - Start parallel: {:?}",now2.elapsed() );
    
    for i in array2.iter() {
        print!("{} ",i);
    }
}