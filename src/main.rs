mod cliente;
mod usuario;

use cliente::Cliente;
use usuario::Usuario;

fn main()
{
    let cli: Cliente = Cliente::new
    (
        String::from("Fernando Costa Migliotini"),
        String::from("fercosmig@gmail.com")
    );

    println!("Cliente => nome:{} e e-mail: {}", cli.nome, cli.email);

    let user: Usuario = Usuario::new
    (
        "Jurandir Migliorini".to_string(),
        "j.migliorini@gmail.com".to_string(),
        "123".to_string()
    );

    println!("Usuário => nome:{}, e-mail: {} e senha: {}", user.nome, user.email, user.senha);
}
