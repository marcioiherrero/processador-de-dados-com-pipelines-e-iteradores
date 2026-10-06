#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct Pessoa {
  nome: String,
  idade: u8,
  cidade: String,
  salario: u32,
}

impl Pessoa {
  fn from_csv(linha: &str) -> Option<Self> {
    // Parse: linha.split(','), converte tipos
    // Retorna None se falhar
    let dados:Vec<&str> = linha.split(',').collect();
    
    let nome = dados.get(0)?;
    let idade = dados.get(1)?;
    let cidade = dados.get(2)?;
    let salario = dados.get(3)?;
    
    let novo = Pessoa {
      nome: nome.to_string(),
      idade: idade.parse::<u8>().unwrap(),
      cidade: cidade.to_string(),
      salario: salario.parse::<u32>().unwrap(),
    };
    
    Some(novo)
  }
}

fn main() {
  let dados = vec![
    "nome,idade,cidade,salario".to_string(),
    "Ana,25,São Paulo,7800".to_string(),
    "Beltrano,51,Florianóplis,4500".to_string(),
    "Ciclana,22,Cuiabá,6300".to_string(),
    "Bruno,30,Rio de Janeiro,6000".to_string(),
    "Tiozinho,63,São Paulo,3100".to_string(),
    "Tia da esquina,71,São Paulo,5000".to_string(),
  ];
  
  let mut pessoas: Vec<Pessoa> = Vec::new();
  
  for dado in dados.iter().skip(1) {
    pessoas.push(Pessoa::from_csv(&dado).unwrap());
  }
  
  let pessoas_sao_paulo = pessoas.iter()
    .filter(|p| p.cidade == "São Paulo")
    .collect::<Vec<&Pessoa>>();
  
  let mut tres_mais_jovens = pessoas.clone();
  tres_mais_jovens.sort_by_key(|p| p.idade);
  let tres_mais_jovens: Vec<Pessoa> = tres_mais_jovens.into_iter()
    .take(3)
    .collect();
  
  // Relatórios
  
  println!("\nTodas as pessoas cadastradas:");
  for p in &pessoas {
    println!("{:?}", p);
  }
  
  println!("\nPessoas da cidade de São Paulo:");
  for p in pessoas_sao_paulo {
    println!("{:?}", p);
  }
  
  println!("\nEstatísticas:");
  let qtde_pessoas = pessoas.len();
  
  let soma_sal = pessoas.iter()
    .fold(0, |acc, p| {
      acc + p.salario
    });
  let sal_medio: f32 = soma_sal as f32 / qtde_pessoas as f32;
  
  let soma_idade: u32 = pessoas.iter()
    .fold(0, |acc, p| {
      acc + p.idade as u32
    });
  let idade_media = soma_idade as u32 / qtde_pessoas as u32;
  
  let maior_sal = pessoas.iter()
    .max_by_key(|p| p.salario).unwrap().salario;
  
  let menor_sal = pessoas.iter()
    .min_by_key(|p| p.salario).unwrap().salario;
  
  println!("Salário médio: {}", sal_medio);
  println!("Idade média: {}", idade_media);
  println!("Maior salário: {}", maior_sal);
  println!("Menor salário: {}", menor_sal);
  
  println!("\n3 pessoas mais jovens:");
  for p in tres_mais_jovens {
    println!("{:?}", p);
  }
  
  let nomes_str = pessoas
    .iter()
    .map(|x| x.nome.to_uppercase())
    .collect::<Vec<_>>()
    .join(",");
  
  println!("\nNomes das pessoas em maiúsculas:\n{}", nomes_str);
}
