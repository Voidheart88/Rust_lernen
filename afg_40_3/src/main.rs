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

use rand::prelude::*;                   //Für den Zufallszahlengenerator
use std::io::{stdin,stdout,Write};      //


fn matrix2_max (matrix: &Vec<f32>) -> (f32,usize){ //Funktionsdeklaration "fn name(argument1: Datentyp1, ...) -> Rückgabetyp {}"
    let mut max = 0.0;
    let mut index: usize=0;

    for i in 0..matrix.len() {
        if max < matrix[i] {
            max = matrix[i];
            index = i;
        }
    }
    return (max, index); //Tupel sind toll
}



fn main() {    
    let mut input_m_s=String::new();
    let mut input_n_s=String::new();

    //Begrüßung
    println!("Matrixmax Programm!");

    //Einlesen M
    print!("Gib einen Wert für M ein: ");
    let _= stdout().flush();
    stdin().read_line(&mut input_m_s).expect("Did not enter a correct string");
    if let Some('\n') = input_m_s.chars().next_back() {
        input_m_s.pop();
    }
    if let Some('\r') = input_m_s.chars().next_back() {
        input_m_s.pop();
    }

    //Einlesen N
    print!("Gib einen Wert für N ein: ");
    let _= stdout().flush();
    stdin().read_line(&mut input_n_s).expect("Did not enter a correct string");
    if let Some('\n')=input_n_s.chars().next_back() {
        input_n_s.pop();
    }
    if let Some('\r')=input_n_s.chars().next_back() {
        input_n_s.pop();
    }

    //Umwandeln von String in usize
    let input_m = input_m_s.parse::<usize>().unwrap();
    let input_n = input_n_s.parse::<usize>().unwrap();

    //Erstellen des Vektors als Repräsentation der Matrix
    let mut mat = vec![0.0;input_m*input_n];

    //Füllen des Vektors mit Zufallszahlen
    for i in 0..mat.len() {
        mat[i] = random::<f32>();
    }

    //Ausgabe des Maximums durch Funktion
    let result = matrix2_max(&mat); 
    println!("Wert, Index: {0},{1}",&result.0,&result.1);
    println!("Stelle in der Matrix - M:{0} N:{1}",&result.1 / input_m, &result.1%input_m);
}
