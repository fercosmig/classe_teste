use crate::cliente::Cliente;

pub struct ClienteController {}

impl ClienteController
{
    pub fn cliente_to_uppercase(mut cliente: Cliente) -> Cliente
    {
        let nome = cliente.nome;
        let email = cliente.email;

        cliente.nome = nome.to_uppercase();
        cliente.email = email.to_uppercase();
        
        return cliente;
    }

    pub fn imprime_teste(x: &String)
    {
        println!("fn imprime_teste() do ClienteController: {}", x);
    }
}