mod cliente;
mod usuario;

fn main()
{
    let cliente = cliente::Cliente
    {
        nome: String::from("Fernando Costa Migliorini"),
        email: String::from("fercosmig@gmai.com")
    };

    println!("{}", cliente.nome);
    println!("{}", cliente.email);

    let usuario = usuario::Usuario
    {
        nome: String::from("Jurandir Migliorini"),
        email: String::from("j.migliorini@gmail.com"),
        senha: String::from("123")
    };

    println!("{}", usuario.nome);
    println!("{}", usuario.email);
    println!("{}", usuario.senha);
}
