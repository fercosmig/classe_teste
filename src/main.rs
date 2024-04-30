mod cliente;
use cliente::Cliente;

mod usuario;
use usuario::Usuario;

mod cliente_controller;
use cliente_controller::ClienteController;

use std::io;

fn main()
{
    let mut cliente: Cliente = Cliente::new
    (
        String::from("Fernando Costa Migliorini"),
        String::from("fercosmig@gmail.com")
    );

    println!("Cliente antes => nome:{} e e-mail: {}", cliente.nome, cliente.email);
   
    cliente = ClienteController::cliente_to_uppercase(cliente);

    println!("Cliente depois => nome:{} e e-mail: {}", cliente.nome, cliente.email);

    let usuario: Usuario = Usuario::new
    (
        "Jurandir Migliorini".to_string(),
        "j.migliorini@gmail.com".to_string(),
        "123".to_string()
    );

    println!("Usuário => nome:{}, e-mail: {} e senha: {}", usuario.nome, usuario.email, usuario.senha);

    ClienteController::imprime_teste(&usuario.email);
}
