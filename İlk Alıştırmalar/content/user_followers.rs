#[derive(Debug)]
pub struct UserFollowers {
    pub followers: u32,
    pub name: String,
}

impl UserFollowers {
    pub fn new(followers: u32, name: String) -> Self {
        UserFollowers {
            followers,
            name,
        }
    }

    pub fn add_followers(&mut self) {
        self.followers += 1;
    }

    pub fn remove_followers(&mut self) {
        if self.followers > 0 {
            self.followers -= 1;
        }
    }
}
