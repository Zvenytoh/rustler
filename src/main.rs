fn main() {
    let mut name = String::from("Mehdi");
    let lecture = &name;
    println!("Lecture :{}", lecture);
    add_title(&mut name);
    let namebis = name.clone();
    println!("Name: {}", &name);
    println!("NameBis: {}", namebis);
}

fn add_title(name: &mut String) {
    name.push_str(" (Mr.)");
}

struct Personne {
    username: String,
    age: u32,
}

impl Personne {
    pub fn new(username: String, age: u32) -> Self {
        Self { username, age }
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn feter_anniversaire(&mut self) {
        self.age += 1
    }
}
