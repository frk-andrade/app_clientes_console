use crate::tela::{ler::{ler_dados, ler_dados_int}, op_basicas::{limpar_tela, esperar}};

pub fn mostrar_menu(){
    
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
            1 => println!("Opção 1 selecionada: Cadastrar cliente"),
            2 => println!("Opção 2 selecionada: Alterar cliente"),
            3 => println!("Opção 3 selecionada: Excluir cliente"),
            4 => println!("Opção 4 selecionada: Listar clientes"),
            5 => {
                println!("Opção 5 selecionada: Sair do programa...");
                return;
            }
            _ => println!("Opção inválida. Tente novamente."),
        }

        // println!("Pressione Enter para continuar...");
        // ler_dados();
        esperar(2);
    }
}