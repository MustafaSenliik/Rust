// User follower structure
struct UserFollowers {
    username: String,
    followers: u32,
}

impl UserFollowers {
    // Display the number of followers
    fn show_followers(&self) {
        println!("Username: {}", self.username);
        println!("Followers: {}", self.followers);
    }

    // Add a follower
    fn add_follower(&mut self) {
        self.followers += 1;
        println!("You gained a new follower! Total followers: {}", self.followers);
    }

    // Remove a follower
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
    // Create a user
    let mut user = UserFollowers {
        username: String::from("mustafasenlik"),
        followers: 100,
    };

    // Display the number of followers
    user.show_followers();

    // Add a new follower
    user.add_follower();

    // Lose a follower
    user.remove_follower();
}
