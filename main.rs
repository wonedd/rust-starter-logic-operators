fn main() {
  let idade: u8 = 18;
  let responsavel_autorizou = true;
  let maior_de_idade = idade >= 18;

  if idade >= 18 {
    println!("Pode entrar na balada");
  }else if idade > 16 && responsavel_autorizou {
    println!("Pode entrar na balada com assinatura do responsável");
  } else {
    println!("Não pode entrar na balada");
  }

  let condicao:&str;

  condicao = if maior_de_idade { "maior" } else { "menor" };

  println!("É {} de idade", condicao);
    
}