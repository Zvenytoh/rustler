use crossterm::terminal::LeaveAlternateScreen;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, BorderType, Borders};
use ratatui::{
    Terminal,
    prelude::{CrosstermBackend, backend},
};
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen);
    let backend = CrosstermBackend::new(&mut stdout);

    let mut ma_liste: Vec<Tache> = Vec::new();
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let zones = layout_app(f.area());

            f.render_widget(
                Block::default()
                    .title("Rustler")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
                zones[0],
            );
        })?;

        // println!("Entrez une tâche (ou 'exit' pour quitter) :");
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
                // println!("\n Ajoutez une tâche pour pouvoir exécuter cette action\n\n ")
            }
        } else {
            let nouvelle_tache = Tache::new(String::from(nom_propre));
            ma_liste.push(nouvelle_tache);
            let i = ma_liste.len() - 1;
            ma_liste[i].statut = Statut::EnCours;
        }
    }
    for tache in &ma_liste {
        // println!(
        // "Statut: {:?} | Description: {}",
        // tache.statut, tache.description
        // );
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
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

fn layout_app(zone_totale: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(60),
            Constraint::Percentage(10),
        ])
        .split(zone_totale)
}
