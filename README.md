# App Clientes Console 🚀

> Uma aplicação interativa em console desenvolvida para o gerenciamento eficiente de cadastros de clientes, focada em boas práticas de programação, estruturação de dados e design patterns.

Este projeto foi desenvolvido com o objetivo de consolidar conceitos fundamentais de desenvolvimento de software, criando uma solução robusta para linha de comando (CLI) que demonstra habilidades essenciais buscadas por recrutadores e equipes técnicas.

---

## 🛠️ Tecnologias e Conceitos Utilizados

*   **Linguagem de Programação:** **RUST**
*   **Paradigma:** Programação Orientada a Objetos (POO)
*   **Persistência de Dados:** **Memória**
---

### 1. Arquitetura Orientada a Objetos (POO)
*   **Abstração e Encapsulamento:** Implementação de classes de entidade (ex: `Cliente`) com propriedades bem protegidas e métodos de validação interna.
*   **Responsabilidade Única (SRP):** Classes de persistência separadas da lógica de exibição no console. O fluxo de tela não interfere nas regras de negócio.

### 2. Gerenciamento Completo de Estado (CRUD)
*   **Create (Cadastrar):** Sistema de captação de dados via console com validações em tempo real (ex: formato de e-mail, CPF/CNPJ, campos obrigatórios).
*   **Read (Listar/Consultar):** Exibição formatada em formato de tabela no terminal, permitindo a busca detalhada por ID, Nome ou Status.
*   **Update (Atualizar):** Fluxo inteligente de edição que permite alterar campos específicos sem a necessidade de reescrever todo o cadastro.
*   **Delete (Remover/Inativar):** Implementação de exclusão lógica (inativação do cliente) ou física, preservando a integridade referencial dos dados.

### 3. Tratamento de Exceções e Resiliência
*   **Prevenção de Crashes:** Toda entrada de dados pelo teclado (como números de telefone, IDs ou datas) é tratada contra erros de digitação comuns (ex: inserção de letras em campos numéricos), utilizando estruturas robustas de conversão (como `int.TryParse` / `try-catch`).
*   **Validações Robustas:** Lógica estruturada para impedir duplicidade de cadastros idênticos no sistema.

### 4. Persistência de Dados Eficiente
*   Estrutura de dados otimizada para busca e armazenamento temporário (Listas, Dicionários) ou gravação direta em arquivos (JSON/CSV) para garantir que as informações não sejam perdidas ao reiniciar o console.

---

## 📂 Estrutura do Projeto

```text
├── src/
│   ├── Model/          # Entidades de dados (ex: Cliente.cs)
│   ├── Repository/     # Lógica de persistência e acesso a dados
│   ├── Service/        # Regras de negócio e validações
│   └── Program.cs      # Ponto de entrada (Fluxo de Telas e Menu Principal)
└── README.md
```

---

## 🚀 Como Executar o Projeto

1. **Clone o repositório:**
   ```bash
   git clone https://github.com/frk-andrade/app_clientes_console.git
   ```
2. **Navegue até a pasta do projeto:**
   ```bash
   cd app_clientes_console
   ```
3. **Execute a aplicação:**
     ```bash
     cargo run
     ```

---

## 💡 Habilidades Demonstradas neste Projeto

*   Domínio de lógica de programação estruturada e tomada de decisão.
*   Manipulação avançada de strings e fluxos de entrada/saída (I/O).
*   Organização de código limpo (Clean Code) com foco em legibilidade e manutenibilidade.
*   Versionamento de código profissional utilizando boas práticas de commits e Git.
