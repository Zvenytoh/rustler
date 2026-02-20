fn main() {
    let mut ma_liste: Vec<Tache> = Vec::new();

    loop {
        println!("Entrez une tâche (ou 'exit' pour quitter) :");
        let mut saisie = String::new();
        std::io::stdin().read_line(&mut saisie).unwrap();
        let nom_propre = saisie.trim();
        if nom_propre == "exit" {
            break;
        } else if nom_propre == "fait" {
            if !ma_liste.is_empty() {
                let i = ma_liste.len() - 1;
                ma_liste[i].statut = Statut::Termine;
            } else {
                println!("\n Ajoutez une tâche pour pouvoir exécuter cette action\n\n ")
            }
        } else {
            let nouvelle_tache = Tache::new(String::from(nom_propre));
            ma_liste.push(nouvelle_tache);
            let i = ma_liste.len() - 1;
            ma_liste[i].statut = Statut::EnCours;
        }
    }
    for tache in &ma_liste {
        println!(
            "Statut: {:?} | Description: {}",
            tache.statut, tache.description
        );
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
