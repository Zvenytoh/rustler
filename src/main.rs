use std::io::{self, Read};

fn main() {
    
    let mut ma_liste: Vec<Tache> = Vec::new();
    
    let t1 = Tache::new(String::from("Faire le premier défi"));
    ma_liste.push(t1);
    
    let t2 = Tache::new(String::from("Deuxième tache"));
    ma_liste.push(t2);
    
    println!("{:?}", ma_liste);
    
    loop {
        println!("Entrez une tâche (ou 'exit' pour quitter) :");
        let mut saisie = String::new();
        std::io::stdin().read_line(&mut saisie).unwrap();
        let nom_propre = saisie.trim();
        if nom_propre == "exit" {
            break;
        }
        let nouvelle_tache = Tache::new(String::from(nom_propre));
        ma_liste.push(nouvelle_tache);
    }
}

#[derive(Debug)]
enum Statut {
    AFaire,
    EnCours,
    Termine,
}

#[derive(Debug)]
struct Tache {
    description: String,
    statut: Statut,
}

impl Tache {
    fn new(tache_name: String) -> Tache {
        return Tache {
            description: tache_name,
            statut: (Statut::AFaire),
        };
    }
}
