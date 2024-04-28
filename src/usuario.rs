pub struct Usuario
{
    pub nome: String,
    pub email: String,
    pub senha: String
}

impl Usuario
{
    pub fn new(nome: String, email: String, senha: String) -> Usuario
    {
        Usuario
        {
            nome,
            email,
            senha
        }
    }
}