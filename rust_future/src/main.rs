extern crate rand;

use rand::prelude::*;               //Für den Zufallszahlengenerator
use std::time::Instant;             //Für Zeitmessungen
use std::f32;                       //Für minimal und Maximalwerte von Gleitkommazahlen


fn calc(&array) -> (f32,f32,f32){
    let mut max: f32 = f32::MIN;
    let mut min: f32 = f32::MAX;
    let mut avg: f32 = 0.0;
    task::spawn(async {
        for i in array{
            if i>max = max = i;
        }
    });

    return (max,min,acg)
}

fn main() {
    let mut array : [f32;100000] = [0.0;100000];        //Arraydeklaration mit 1000 Feldern [Datentyp;Anzahl] initialisiert mit 0.0
                                                        //Alternative: Listendeklaration [0,1,2,3,4,5,7...] (wäre nur etwas anstrengend) 
                                                        //TODO herausfinden ob man das Array direkt mit Zufallszahlen initialisieren kann
    let mut rand_max: f32 = f32::MIN;                   //Enthält den Maximalwert im Array -initialisiert als Minimum von f32
    let mut rand_min: f32 = f32::MAX;                   //Enthält den Minimalwert im Array -initialisiert als Maximum von f32
    let mut average: f32 = 0.0;                         //Enthält den Durchschnitt  

    //Fülle das Array mit Werten
    for i in 0..array.len(){
        array[i] = random::<f32>();         //speichere eine Zufallszahl an die Stelle des Arrays
    }

    /*________________________________________*/

    //Versuch ohne Parallelbearbeitung
    let mut now = Instant::now();
    println!("Zeit vergangen - Start normal: {:?}",now.elapsed() );

    for i in 0..array.len(){
        if rand_max < array[i] {rand_max = array[i];}
        if rand_min > array[i] {rand_min = array[i];}
        average += array[i];
    }
    average /= array.len() as f32; //sollte gegen 0.5 gehen
    let t_norm = now.elapsed();
    println!("Maximum im Array: {}", rand_max );
    println!("Minimum im Array: {}", rand_min );
    println!("Durchschnitt: {}", average );
    println!("Zeit vergangen - Start normal: {:?}",now.elapsed() );

    //Versuch mit Parallelbearbeitung
    now = Instant::now();
    let (max,min,avg) = calc(array);


    let t_fut = now.elapsed();
    println!("Maximum im Array: {}", rand_min);
    println!("Minimum im Array: {}", rand_min);
    println!("Durchschnitt: {}", rand_min);
    println!("Zeit vergangen - Start normal: {:?}",now.elapsed() );

}
