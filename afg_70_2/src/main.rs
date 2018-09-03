/*
Entwickeln sie für den komplexen Kehrwert 1/z ein weiteres Unterprogramm.
Nutzen Sie dieses und die schon bekannten Unterprogramme um den komplexen
Gesamtwiderstand von Wechselstromschaltungen zu Berechnnen.

Erstellen sie eine Tabelle die die Wechselstromwiderstände in Abhängigkeit Frequenz f darstellt.
f: [10khz,680khz]

Parallelkapazität   5.0e-9f  Farad
Reihenwiderstand    50.0f    Ohm   
Reiheninduktivität  80.0e-6f Henry
Reihenkapazität     12.5e-9f Farad
*/
use std::ops;

const CAP_P : f32 = 5.0e-9;     //Parallelkapazität
const RES_S : f32 = 50.0;       //SerienWiderstand
const IND_S : f32 = 80.0e-6;    //Serieninduktivität
const CAP_S : f32 = 12.5e-9;    //Serienkapazität

struct Complex {
    re: f32,
    im: f32,
}

impl Complex {
    fn add(self,sum: Complex ) -> Complex  {
        let mut result: Complex = Complex{re:0.0, im:0.0};
        result.re = self.re + sum.re;
        result.im = self.im + sum.im;
        return result;
    }
}

impl ops::Add<Complex> for Complex {
    type Output = Complex;
    fn add(self,sum: Complex ) -> Complex  {
        let mut result: Complex = Complex{re:0.0, im:0.0};
        result.re = self.re + sum.re;
        result.im = self.im + sum.im;
        return result;
    }
}

fn main() {
    println!("Z in Abh. von F");
    for i in 1..68{
        let f = (i as f32)*10000.0;
        let z_p: Complex = Complex{re:0.0, im:1.0/(2.0*std::f32::consts::PI*f*CAP_P)};
        let z_r_s: Complex = Complex{re:RES_S   , im: 0.0};
        let z_i_s: Complex = Complex{re:0.0     , im: 2.0*std::f32::consts::PI*f*IND_S};
        let z_c_s: Complex = Complex{re:0.0     , im: -1.0/(2.0*std::f32::consts::PI*f*CAP_S)};
        let z_s = z_r_s + z_i_s + z_c_s;

        print!("z_p = {},{} ",z_p.re,z_p.im);
        println!("z_s = {},{}",z_s.re,z_s.im);
    }
}
