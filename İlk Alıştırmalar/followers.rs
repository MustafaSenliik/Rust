// Kullanıcı takipçi yapısı
struct UserFollowers {
    username: String,
    followers: u32,
}

impl UserFollowers {
    // Kullanıcının takipçi sayısını görüntüleme
    fn show_followers(&self) {
        println!("Username: {}", self.username);
        println!("Followers: {}", self.followers);
    }

    // Takipçi ekleme
    fn add_follower(&mut self) {
        self.followers += 1;
        println!("You gained a new follower! Total followers: {}", self.followers);
    }

    // Takipçi çıkarma
    fn remove_follower(&mut self) {
        if self.followers > 0 {
            self.followers -= 1;
            println!("You lost a follower! Total followers: {}", self.followers);
        } else {
            println!("No followers to remove!");
        }
    }
}

fn main() {
    // Kullanıcı oluşturma
    let mut user = UserFollowers {
        username: String::from("mustafasenlik"),
        followers: 100,
    };

    // Takipçi sayısını gösterme
    user.show_followers();

    // Yeni bir takipçi ekleme
    user.add_follower();

    // Bir takipçi kaybetme
    user.remove_follower();
}
