use std::io::stdin;
use crate::student::{StudentList};

mod student;

// Macro to clean input and read line from keyboard
macro_rules! read_line {
    ($i:expr) => {
        $i.clear();
        stdin().read_line(&mut $i).expect("Falha ao ler a string");
    };
}


fn main() {
    let mut input : String = String::new();
    let mut student_list :StudentList = StudentList::new();

    // Valores temporários utilizados nas caixas de seleção do menu
    let mut name : student::Name;
    let mut age : student::Age;
    let mut grade : student::Grade;



    'main: loop {
        println!("Atualmente alocado: {}. Tamanho máximo: {}", student_list.len(), student_list.capacity());
        println!("\x1b[2J\x1b[HPergunta:
        1 - Criar uma coleção
        2 - Inserir elementos na coleção
        3 - Listar os elementos da coleção
        4 - Consultar um dos elementos da coleção
        5 - Remover algum dos elementos da coleção
        6 - Esvaziar a coleção
        7 - Destruir a coleção
        8 - Sair
        ");

        // Limpa a input pra entrada no próximo read_line()
        read_line!(input);

        match input.trim().parse::<u8>() {
            Ok(1) => {
                // Criar coleção
                // Pergunta a quantidade de caracteres
                'quant: loop {
                    println!("Digite a quantidade de itens para pré-alocar");
                    read_line!(input);
                    match input.trim().parse::<u8>() {
                        Ok(num) => {
                            student_list = StudentList::with_capacity(num);
                            break 'quant;
                        },
                        Err(_) => {
                            println!("Por favor digite um valor válido e entre 0 e 254");
                        }
                    }
                }
            },
            Ok(2) => {
                // Inserir elementos na coleção
                'data: loop {
                    println!("Digite o nome do indivíduo");
                    read_line!(input);

                    match student::Name::try_from_str(input.clone()) {
                        Ok(result) => {
                            name = result;
                            break 'data;
                        }
                        Err(_) => {
                            println!("Nome muito grande. O limite é 30 caracteres");
                        }
                    }
                }

                'data: loop {
                    println!("Digite a idade do indivíduo");
                    read_line!(input);

                    match input.trim().parse::<student::Age>() {
                        Ok(result) => {
                            age = result;
                            break 'data;
                        }
                        Err(_) => {
                            println!("Verifique a idade digitada");
                        }
                    }
                }

                'data: loop {
                    println!("Digite a nota do indivíduo");
                    read_line!(input);

                    match input.trim().parse::<student::Grade>() {
                        Ok(result) => {
                            grade = result;
                            break 'data;
                        }
                        Err(_) => {
                            println!("Verifique a nota digitada");
                        }
                    }
                }

                match student_list.add(name, age, grade) {
                    Ok(_) => {}
                    Err(err) => {
                        println!("{}", err);
                    }
                };
            },
            Ok(8) => {
                break 'main;
            },
            Ok(_) => {
                println!("Opção desconhecida")
            },
            Err(_) => {
                println!("Por favor insira um número válido");
            }
        }
    }

    // let mut students: student::StudentList = student::StudentList::new(5);
    // students.add(String::from("Aluno1"), 12, 2.5).expect("Erro ao adicionar aluno");
    // students.add(String::from("Aluno2"), 12, 2.5).expect("Erro ao adicionar aluno");

}
