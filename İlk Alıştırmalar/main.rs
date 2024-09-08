#[derive(Debug)]
struct Deste{
    kartlar: Vec<String>
}
 impl Deste {
     fn new() -> Self{
        let mut kartlar = vec![]; 
    let kart_turleri = ["Kupa", "Karo","Sinek"];
    let degerler = ["2","3","4"];

    for kart_turleri in kart_turleri{
        for degerler in degerler{
            let kart = format!("{} {}", kart_turleri, degerler);
            kartlar.push(kart);
        }
    }
        

    let deste:Deste = Deste{kartlar};
    return deste;
     }
     
 }
fn main(){
    
    let deste = Deste::new();
    
    println!("Deste Burada: {:#?}",deste);
}
