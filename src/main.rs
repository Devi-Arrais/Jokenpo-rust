use std::io;
use rand::Rng;

enum Jokenpo{
    Papel,
    Tesoura,
    Pedra
}

struct Resultado{
    ganhador: String,
    player1: String,
    player2: String,
    empate: bool,
}
impl Resultado{
    fn ganhador(&mut self){
        if self.empate == true{
            println!("Empate")
        }else{
            println!("O ganhdor foi {}, Voce escolheu {} o computador {}", self.ganhador, self.player2, self.player1)
        }
    }
}

fn main() {
    println!("Pedra, Papel ou Tesoura:");
    let mut ln = String::new(); 
    io::stdin().read_line(&mut ln).unwrap();
    player(ln);
    
}    

fn player(jogada: String){
    let play: &str = &jogada.trim().to_uppercase();
    match play{
        "PAPEL" => computador(Jokenpo::Papel),
        "TESOURA" => computador(Jokenpo::Tesoura),
        "PEDRA" => computador(Jokenpo::Pedra),
        _ => println!("Escolha entre Pedra, Papel, Tesoura"),
    };
}

fn computador(play: Jokenpo){
    let cpu = rand::thread_rng().gen_range(1..=3);
    match cpu{
        1 => resultado(Jokenpo::Pedra, play),
        2 => resultado(Jokenpo::Papel, play),
        3 => resultado(Jokenpo::Tesoura, play),
        _ =>todo!(),
    };
}
 
fn resultado(cpu: Jokenpo, play: Jokenpo){
    let mut ganha = match(cpu, play){
        (Jokenpo::Papel, Jokenpo::Pedra) => Resultado{ganhador: "CPU".to_string(), player1: "Papel".to_string(), player2: "Pedra".to_string(), empate: false},
        (Jokenpo::Pedra, Jokenpo::Papel) => Resultado{ganhador: "Jogador".to_string(), player1: "Pedra".to_string(), player2: "Papel".to_string(), empate: false},
        (Jokenpo::Tesoura, Jokenpo::Papel) => Resultado{ganhador: "CPU".to_string(), player1: "Tesoura".to_string(), player2: "Papel".to_string(), empate: false},
        (Jokenpo::Papel, Jokenpo::Tesoura) => Resultado{ganhador: "Jogador".to_string(), player1: "Papel".to_string(), player2: "Tesoura".to_string(), empate: false},
        (Jokenpo::Pedra, Jokenpo::Tesoura) => Resultado{ganhador: "CPU".to_string(), player1: "Pedra".to_string(), player2: "Tesoura".to_string(), empate: false},
        (Jokenpo::Tesoura, Jokenpo::Pedra) => Resultado{ganhador: "Jogador".to_string(), player1: "Tesoura".to_string(), player2: "Pedra".to_string(), empate: false},
        (_,_) => Resultado{ganhador: "empate".to_string(), player1: "empate".to_string(), player2: "empate".to_string(), empate: true},
    };
    Resultado::ganhador(&mut ganha);
}
