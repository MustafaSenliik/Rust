use rand::{thread_rng, seq::SliceRandom}; 

#[derive(Debug)]
struct Deste {
    kartlar: Vec<String>,
}

impl Deste {
    fn new() -> Self {
        let mut kartlar = vec![];
        let kart_turleri = ["Kupa", "Karo", "Sinek"];
        let degerler = ["2", "3", "4"];

        for kart_turleri in kart_turleri {
            for degerler in degerler {
                let kart = format!("{} {}", kart_turleri, degerler);
                kartlar.push(kart);
            }
        }

        Deste { kartlar }
    }

    // Shuffle fonksiyonu burada tanımlanmalı
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.kartlar.shuffle(&mut rng); // SliceRandom kullanarak shuffle yapıyoruz
    }
}

fn main() {
    let mut deste = Deste::new(); // Deste mutably oluşturulmalı ki shuffle edilebilsin
    deste.shuffle(); // Shuffle işlemi

    println!("Deste Burada: {:#?}", deste); // Deste yazdırılıyor
}
