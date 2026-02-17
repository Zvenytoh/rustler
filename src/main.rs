fn main() {
    let p = Personne::new(String::from("Mehdi"), 30);

    match p {
        Some(mut p_interne) => {
            let nom_alias = p_interne.get_username();
            println!("{}", nom_alias);
            p_interne.feter_anniversaire();
            println!("Age de la personne interne : {}", p_interne.age);
        }
        None => println!("La personne interne n'existe pas"),
    };
}

struct Personne {
    username: String,
    age: u32,
}

impl Personne {
    pub fn new(username: String, age: u32) -> Option<Self> {
        if age < 120 {
            Some(Self { username, age })
        } else {
            None
        }
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn feter_anniversaire(&mut self) {
        self.age += 1
    }
}
