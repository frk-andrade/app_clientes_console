use crate::models::cliente::{self, Cliente};
use crate::tela::ler::{ler_dados, ler_dados_int};
use crate::tela::op_basicas::{esperar, limpar_tela};

pub fn incluir_cliente(clientes: &mut Vec<Cliente>) {
    limpar_tela();

    let mut cliente: Cliente = Cliente::default();

    cliente.id = clientes.len() + 1;
   
    digitar_dados_cliente(&mut cliente);

    clientes.push(cliente);
    limpar_tela();
    println!("Cliente cadastrado com sucesso!");
    esperar(1);
}

fn digitar_dados_cliente(cliente: &mut Cliente){
    println!("Digite o nome do cliente:");
    cliente.nome = ler_dados();
    println!("Digite o CPF do cliente:");
    cliente.cpf = ler_dados();
    println!("Digite o endereço do cliente:");
    cliente.endereco = ler_dados();
}

pub fn alterar_cliente(clientes: &mut Vec<Cliente>) {
    limpar_tela();
     if nao_tem_clientes(clientes) {
        return;
    }
    
    let id = captura_id();
    if let Some((indice, cliente)) = buscar_cliente_por_id(clientes, id) {
        println!("{}", "-".to_string().repeat(40));
        println!("Alterando o cliente");
        println!("{}", "-".to_string().repeat(40));
        mostrar_cliente(cliente);
        println!("{}", "-".to_string().repeat(40));
        digitar_dados_cliente(&mut clientes[indice]);
        limpar_tela();
        println!("Cliente alterado com sucesso!");

    } else {
    limpar_tela();
    println!("Cliente não encontrado!");
    }

    esperar(1);
}

fn buscar_cliente_por_id(clientes: &Vec<Cliente>, id: usize) -> Option<(usize, &Cliente)> {
    clientes.iter().enumerate().find(|(_, cliente)| cliente.id == id)
}

fn captura_id() -> usize {
    limpar_tela();
    println!("Digite o ID do cliente que deseja alterar:");
    ler_dados_int()
}

pub fn listar_clientes(clientes: &mut Vec<Cliente>) {
    limpar_tela();

    if nao_tem_clientes(clientes) {
        return;
    }

    if clientes.len() == 0 {
        println!("Nenhum cliente cadastrado.");
        esperar(1);
        return;
    }

    println!("{}", "-".to_string().repeat(40));
    for cliente in clientes {
        mostrar_cliente(cliente);
        println!("{}", "-".to_string().repeat(40));
    }

    println!("Pressione Enter para continuar...");
    ler_dados();
}

fn nao_tem_clientes(clientes: &[Cliente]) -> bool {
    if clientes.len() == 0 {
        println!("Nenhum cliente cadastrado.");
        esperar(1);
        return true;
    }
    false
}

fn mostrar_cliente(cliente: &Cliente) {
    println!("ID: {}", cliente.id);
    println!("Nome: {}", cliente.nome);
    println!("CPF: {}", cliente.cpf);
    println!("Endereço: {}", cliente.endereco);
}
