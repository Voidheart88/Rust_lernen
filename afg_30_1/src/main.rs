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
use std::thread;
use std::sync::mpsc::channel;

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap(); //erstelle den Threadpool

    let mut array : [f32;10000] = [0.0;10000];  //Arraydeklaration mit 1000 Feldern [Datentyp;Anzahl] initialisiert mit 0.0
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


    /*________________________________________*/
    //Versuch ohne Parallelbearbeitung
    let now = Instant::now();
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
    println!("Index: {} \n", index_dev);


    /*________________________________________*/
    //Versuch mit Parallelbearbeitung - Rayon:

    let now = Instant::now(); //Beginne Zeitmessung    
    println!("Zeit vergangen - Start Rayon: {:?}",now.elapsed() );

    let ray_max = array.par_iter().map(|b|*b as f32).reduce(||f32::MIN,|a, b| a.max(b)) ; //Suche das Maximum
    let ray_min = array.par_iter().map(|b|*b as f32).reduce(||f32::MAX,|a, b| a.min(b)) ; //Suche das Minimum    
    let mut ray_avg: f32 = array.par_iter().sum(); //addiere Werte und berechne den Durchschnitt
    ray_avg /= array.len() as f32;
    let ray_max_dev = 0.0;

    let t_rayon = now.elapsed();
    println!("Maximum im Array: {}", ray_max);
    println!("Minimum im Array: {}", ray_min);
    println!("Durchschnitt: {}", ray_avg );

    /*________________________________________*/
    //Versuch mit Parallelbearbeitung - Threads:
    let (th_min_tx, th_min_rx) = channel::<f32>();
    let (th_max_tx, th_max_rx) = channel::<f32>();
    let (th_avg_tx, th_avg_rx) = channel::<f32>();
    let (th_max_dev_tx, th_max_dev_rx) = channel::<f32>();

    let now = Instant::now(); //Beginne Zeitmessung    
    println!("\nZeit vergangen - Start Threads: {:?}",now.elapsed() );

    let array1 = array.clone();
    let t1 = thread::spawn(move ||{        
        th_min_tx.send(array1.iter().fold(f32::MAX, |a, &b| a.min(b)));
    });

    let array2 = array.clone();
    let t2 = thread::spawn(move ||{
        th_max_tx.send(array2.iter().fold(f32::MIN, |a, &b| a.max(b)));
    });

    let array3 = array.clone();
    let t3 = thread::spawn(move ||{
        let mut avg = array3.iter().sum();
        avg /= array3.len() as f32;
        th_avg_tx.send(avg);
    });

    t1.join();
    t2.join();
    t3.join();

    let t_threads = now.elapsed();
    println!("Maximum im Array: {}", th_max_rx.recv().unwrap());
    println!("Minimum im Array: {}", th_min_rx.recv().unwrap());
    println!("Durchschnitt: {}", th_avg_rx.recv().unwrap());

    println!("\nZeit normal: {:?}",t_norm);
    println!("Zeit rayon: {:?}",t_rayon);
    println!("Zeit threads: {:?}",t_threads);
    
}