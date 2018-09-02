/*
Programm DV2_04.C:
Array's und Zeiger Übergabeparameter von Funktionen
maximales Element einer Matrix bestimmen.

Beschreibung Funktion:
Eine Matrix wird im Hauptprogramm alloziiert gefüllt und an eine Funktion übergeben.
Die Funktion sucht das größte Element und gibt den Maximalwert sowie die Indizes bzw. Zeiger auf dieses
an das Hauptprogramm zurück.
*/

extern crate rand;

use rand::prelude::*;               //Für den Zufallszahlengenerator

fn matrix_max (vector: &Vec<f32>) -> f32{ //Funktionsdeklaration "fn name(argument1: Datentyp) -> Rückgabetyp"
    let mut max = 0.0;
    for i in 0..vector.len() {
        if max < vector[i] {
            max = vector[i];
        }
    }
    return max;
}

fn main() {
    let mut vec = vec![0.0;10*5];
    for i in 0..vec.len() {
        vec[i] = random::<f32>();
    }
    println!("Maximum: {}",matrix_max(&vec));
}
