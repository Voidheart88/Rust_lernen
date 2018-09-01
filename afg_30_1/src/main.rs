/*
Aufgabenstellung:
Array mit 100 Float Zahlen erstellen.
Arrayzugriff auf 3 Arten:
-Indizes x[i]
-Berechnete Zeiger *(x+i)
-Inkrementierter Zeiger x++
Berechnung von:
-Maximum
-Minimum
-Index des Elementes mit der Größten Abweichung vom Durchschnitt
-Standartabweichung
*/
extern crate rand;

use rand::prelude::*;               //für den Zufallszahlengenerator
use std::time::{Duration, Instant}; //für Zeitmessungen
use std::f32;                       //für minimal und Maximalwerte von Gleitkommazahlen

fn main() {
    let rng = rand::thread_rng();           //Erstelle eine Instanz des Zufallszahlengenerators
    let mut array : [f32;100] = [0.0;100];  //Arraydeklaration mit 100 Feldern [Datentyp;Anzahl] initialisiert mit 0.0
                                            //Alternative: Listendeklaration [0,1,2,3,4,5,7...] (wäre nur etwas anstrengend) 
                                            //TODO herausfinden ob man das Array direkt mit Zufallszahlen initialisieren kann
    let mut rand_max: f32=f32::MIN;             //Enthält den Maximalwert im Array -initialisiert als Minimum von f32
    let mut rand_min: f32=f32::MAX;             //Enthält den Minimalwert im Array -initialisiert als Maximum von f32
    let mut average:  f32=0.0;                  //Enthält den Durchschnitt    
    let mut index_dev: u32=0;                   //Enthält den Index an dem die größte Abweichung ist

    let mut now = Instant::now();

    //Versuch ohne Concurrency:
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
    average /= 100.0; //sollte gegen 0.5 gehen
    for i in 0..array.len(){
        let x = array[i] - average;
    }
    println!("Maximum im Array: {}", rand_max );
    println!("Minimum im Array: {}", rand_min );
    println!("Durchschnitt: {}", average );
    println!("Zeit vergangen - nach Auswertung: {:?}",now.elapsed() );
}