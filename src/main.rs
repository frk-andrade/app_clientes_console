mod models;

use models::cliente::Cliente;

use crate::models::cliente;


fn main() {
    let cliente: Cliente = Cliente{
        id: 1,
        nome: "teste".to_string(),
        cpf: "12345678900".to_string(),
        endereco: "Rua teste".to_string(),
    };

    println!("Cliente: {}", cliente.nome);
}