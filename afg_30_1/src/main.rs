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

use rayon::prelude::*;
use rand::prelude::*;               //Für den Zufallszahlengenerator
use std::time::Instant;             //Für Zeitmessungen
use std::f32;                       //Für minimal und Maximalwerte von Gleitkommazahlen

fn main() {
    let mut array : [f32;3] = [0.0;3];  //Arraydeklaration mit 1000 Feldern [Datentyp;Anzahl] initialisiert mit 0.0
                                              //Alternative: Listendeklaration [0,1,2,3,4,5,7...] (wäre nur etwas anstrengend) 
                                              //TODO herausfinden ob man das Array direkt mit Zufallszahlen initialisieren kann
    let mut rand_max: f32 = f32::MIN;         //Enthält den Maximalwert im Array -initialisiert als Minimum von f32
    let mut rand_min: f32 = f32::MAX;         //Enthält den Minimalwert im Array -initialisiert als Maximum von f32
    let mut average: f32 = 0.0;               //Enthält den Durchschnitt 
    let mut max_dev: f32 = f32::MIN;          //Enthält den Wert der maximalen Abweichung
    let mut index_dev: usize = 0;             //Enthält den Index an dem die größte Abweichung ist    

    //Fülle das Array mit Werten
    for i in 0..array.len(){
        array[i] = random::<f32>();         //speichere eine Zufallszahl an die Stelle des Arrays
    }

    //Versuch ohne Parallelbearbeitung
    let now = Instant::now();
    println!("Zeit vergangen - Start normal: {:?}",now.elapsed() );

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

    //Versuch mit Parallelbearbeitung:

    let now = Instant::now(); //Beginne Zeitmessung    
    println!("Zeit vergangen - Start parallel: {:?}",now.elapsed() );

    let par_min = array.par_iter().fold(||f32::MAX,|a, &b| a.min(b)); //Suche das Minimum
    let par_max = array.par_iter().fold(||f32::MIN,|a, &b| a.max(b)); //Suche das Minimum
    println!("Zeit vergangen - Ende parallel: {:?}",now.elapsed() );
    println!("Maximum im Array: {:?}", par_min );
    println!("Minimum im Array: {:?}", par_max );

}