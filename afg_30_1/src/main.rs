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
use std::sync::{Arc, Mutex};

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap(); //erstelle den Threadpool

    let mut array : [f32;1000] = [0.0;1000];  //Arraydeklaration mit 1000 Feldern [Datentyp;Anzahl] initialisiert mit 0.0
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
    //Berechnung der maximalen Abweichung
    for i in 0..array.len(){
        let dev = array[i] - average;   //Berechnung der aktuellen differenz
        if dev.abs() > max_dev {        //Wenn die Abweichung größer als die bisherige maximale Abweichung war,
            max_dev = dev.abs();        //aktualisiere die Abweichung
            index_dev = i;              //aktualisiere den Index
        }
    }
    println!("Zeit vergangen - ende Normal: {:?} \n",now.elapsed());
    println!("Maximum im Array: {}", rand_max );
    println!("Minimum im Array: {}", rand_min );
    println!("Durchschnitt: {}", average );
    println!("Maximale Abweichung: {}", max_dev);
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

    println!("Zeit vergangen - Ende Rayon: {:?} \n",now.elapsed());
    println!("Maximum im Array: {}", ray_max);
    println!("Minimum im Array: {}", ray_min);
    println!("Durchschnitt: {}", ray_avg );
    println!("Maximale Abweichung: {} \n", ray_max_dev);

    /*________________________________________*/
    //Versuch mit Parallelbearbeitung - Threads:
    let th_min = Arc::new(Mutex::new(f32::MAX));
    let th_max = Arc::new(Mutex::new(f32::MIN));
    let th_avg = Arc::new(Mutex::new(0.0));
    let th_max_dev = Arc::new(Mutex::new(0.0));

    let now = Instant::now(); //Beginne Zeitmessung    
    println!("Zeit vergangen - Start Threads: {:?}",now.elapsed() );

    let array1 = array.clone();
    let t1 = thread::spawn(||{
        let mut min = th_min.clone().lock().unwrap();
        *min = array1.iter().fold(f32::MAX, |a, &b| a.min(b));
    });

    let array2 = array.clone();
    let t2 = thread::spawn(||{
        let mut max = th_max.clone().lock().unwrap();
        *max = array2.iter().fold(f32::MIN, |a, &b| a.max(b));
    });

    let array3 = array.clone();
    let t3 = thread::spawn(||{
        let mut avg = th_avg.clone().lock().unwrap();
        *avg = array3.iter().sum();
        *avg /= array.len() as f32;
    });
    let now = Instant::now(); //Beginne Zeitmessung

    t1.join();
    t2.join();
    t3.join();

    let th_max_v = th_max.lock().unwrap();
    let th_min_v = th_min.lock().unwrap();
    let th_avg_v = th_avg.lock().unwrap();


    println!("Zeit vergangen - Ende Threads: {:?} \n",now.elapsed());
    println!("Maximum im Array: {}", th_max_v);
    println!("Minimum im Array: {}", th_min_v);
    println!("Durchschnitt: {}", th_avg_v );
    
}