pub struct Cliente
{
    pub nome: String,
    pub email: String    
}

impl Cliente
{
    pub fn new(nome: String, email: String) -> Cliente
    {
        Cliente
        {
            nome,
            email         
        }
    }
}
