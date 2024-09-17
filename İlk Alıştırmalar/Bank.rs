#[derive(Debug)]
struct Ogrenci {
    numara: u32,
    veli: String,
}

impl Ogrenci {
    fn new(numara: u32, veli: String) -> Self {
        Ogrenci {
            numara,
            veli,
        }
    }
}

#[derive(Debug)]
struct Okul {
    ogrenciler: Vec<Ogrenci>,
}

impl Okul {
    fn new() -> Self {
        Okul {
            ogrenciler: vec![],
        }
    }
}

fn print_ogrenci(ogrenci: Ogrenci) -> Ogrenci {  
    println!("{:#?}", ogrenci);
    ogrenci  // Geri döndürülüyor
}

fn print_okul(okul: Okul) {  
    println!("{:#?}", okul);
}

fn main() {
    let ogrenci = Ogrenci::new(1, String::from("ahmet"));
    let okul = Okul::new();
 
    print_okul(okul);  // Okul taşınır ve artık kullanılamaz

    let ogrenci = print_ogrenci(ogrenci);

    println!("{:#?}", ogrenci.veli);
}
