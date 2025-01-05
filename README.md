# Rust Axum - Mini CRUD de Usuário & Evento com JWT request
## Descrição
Projeto simples em Rust usando Axum para gerenciamento de usuários (register/login) e eventos. Faz CRUD de eventos e retorna tokens JWT para autenticação.

## Observações
Os dados são armazenados em memória (HashMap com Mutex), então tudo se perde se reiniciar o servidor.
Teste as rotas via curl ou qualquer cliente REST (Postman, Insomnia etc.).
