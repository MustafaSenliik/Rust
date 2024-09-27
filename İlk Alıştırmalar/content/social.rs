use crate::content::user_followers::UserFollowers;

#[derive(Debug)]
pub struct Social {
    pub followers: Vec<UserFollowers>,  // Social struct'ı, UserFollowers yapısını tutar
}

impl Social {
    pub fn new() -> Self {
        Social {
            followers: vec![],
        }
    }

    pub fn add_social(&mut self, follower: UserFollowers) {
        self.followers.push(follower); // Takipçi ekleme işlemi
    }

    pub fn print_followers(&self) {
        for follower in &self.followers {
            println!("Followers: {}", follower.followers);
        }
    }
}
