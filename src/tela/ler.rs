use std::io;

pub fn ler_dados() -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Falha ao ler a entrada");
    entrada.trim().to_string()
}

pub fn ler_dados_int() -> i32 {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Falha ao ler a entrada");
    entrada.trim().parse().expect("Falha ao converter para inteiro")
}