mod content;  // content modülünü dahil ediyoruz

use content::social::Social;
use content::user_followers::UserFollowers;

fn main() {
    let mut followers = UserFollowers::new(1, String::from("Mustafa Şenlik"));
    let mut social = Social::new();

    followers.add_followers();  // Takipçi sayısını arttırıyoruz
    social.add_social(followers);  // Social'a takipçi ekliyoruz

    println!("{:#?}", social);  // Eklenen takipçiyi göstermek için

    // Social'daki tüm takipçileri yazdırıyoruz
    social.print_followers();
}
