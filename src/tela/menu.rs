use crate::{models::cliente::Cliente, tela::{ler::{ler_dados, ler_dados_int}, op_basicas::{esperar, limpar_tela}, servico_cliente::*}};

pub fn mostrar_menu(clientes: &mut Vec<Cliente>){
    
    loop {

        limpar_tela();

        println!("\
            ==========================Menu=========================\n\
            Escolha uma das opções abaixo:\n\n\
            1 - Cadastrar cliente\n\
            2 - Alterar cliente\n\
            3 - Excluir cliente\n\
            4 - Listar clientes\n\
            5 - Sair do programa\n\
        ");

        let opcao = ler_dados_int();
        limpar_tela();
        match opcao {
            1 => incluir_cliente(clientes),
            2 => println!("Opção 2 selecionada: Alterar cliente"),
            3 => println!("Opção 3 selecionada: Excluir cliente"),
            4 => listar_clientes(clientes),
            5 => {
                println!("Opção 5 selecionada: Sair do programa...");
                return;
            }
            _ => println!("Opção inválida. Tente novamente."),
        }

        // println!("Pressione Enter para continuar...");
        // ler_dados();
    }
}