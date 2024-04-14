use crate::student::{StudentList, SearchOption, Name, Age, Grade};
use crate::menu::MainOption;
use std::io::{stdin, stdout, Write};

mod student;
mod menu;

// Macro to clean input and read line from keyboard
macro_rules! read_line {
    ($i:expr) => {
        stdout().flush().unwrap();
        $i.clear();
        stdin().read_line(&mut $i).expect("Falha ao ler a string");
    };
}

fn main() {
    let mut input: String = String::new();
    let mut student_list: StudentList = StudentList::new();

    // Valores temporários utilizados nas caixas de seleção do menu
    let mut name: Name;
    let mut age: Age;
    let mut grade: Grade;

    'main: loop {
        println!(
            "\x1b[2J\x1b[HStatus da coleção:\n    Atualmente alocado: {}. Tamanho máximo: {}",
            student_list.len(),
            student_list.capacity()
        );
        print!(
            "- Menu Principal -
    1 - Criar uma coleção
    2 - Inserir elementos na coleção
    3 - Listar os elementos da coleção
    4 - Consultar um dos elementos da coleção
    5 - Remover algum dos elementos da coleção
    6 - Esvaziar a coleção
    7 - Destruir a coleção
    8 - Sair\nOpção selecionada: "
        );

        // Limpa a input pra entrada no próximo read_line()
        read_line!(input);

        match input.trim().parse::<MainOption>(){
            Ok(MainOption::Create) => 'early: {
                // Criar coleção se esta está vazia
                if !student_list.is_empty() {
                    println!("Erro: Existem estudantes na lista. Não podemos criar uma nova");
                    break 'early;
                }

                // Pergunta a quantidade de caracteres
                'quant: loop {
                    print!("Digite a quantidade de itens para pré-alocar: ");
                    read_line!(input);
                    match input.trim().parse::<u8>() {
                        Ok(num) => {
                            student_list = StudentList::with_capacity(num);
                            println!("Coleção com {} elementos criada com sucesso!\x1b[K", student_list.capacity());
                            break 'quant;
                        }
                        Err(_) => {
                            print!("Erro: Por favor digite um valor válido e entre 0 e 254 \x1b[1F\x1b[K");
                        }
                    }
                }
            }
            Ok(MainOption::Insert) => 'early: {
                // Insere elementos na coleção se ela existe e não está cheia
                if student_list.len() >= student_list.capacity() {
                    println!("Erro: Não tem espaço suficiente na lista");
                    break 'early;
                }

                'data: loop {
                    print!("Digite o nome do indivíduo: \x1b[K");
                    read_line!(input);

                    match Name::try_from_str(input.trim()) {
                        Ok(result) => {
                            name = result;
                            break 'data;
                        }
                        Err(_) => {
                            print!("Erro: Nome muito grande. O limite é 30 caracteres\x1b[1F\x1b[K");
                        }
                    }
                }

                'data: loop {
                    print!("Digite a idade do indivíduo: \x1b[K");
                    read_line!(input);

                    match input.trim().parse::<Age>() {
                        Ok(result) => {
                            age = result;
                            break 'data;
                        }
                        Err(_) => {
                            print!("Verifique a idade digitada\x1b[1F\x1b[K");
                        }
                    }
                }

                'data: loop {
                    print!("Digite a nota do indivíduo: \x1b[K");
                    read_line!(input);

                    match input.trim().parse::<Grade>() {
                        Ok(result) => {
                            grade = result;
                            break 'data;
                        }
                        Err(_) => {
                            print!("Verifique a nota digitada\x1b[1F\x1b[K");
                        }
                    }
                }

                match student_list.add(name, age, grade) {
                    Ok(_) => println!("Aluno {name} com idade {age} e nota {grade} adicionado a coleção!\x1b[K"),
                    Err(err) => println!("{}", err)
                };
            }
            Ok(MainOption::Show) => {
                if student_list.is_empty() {
                    println!("Erro: Lista vazia");
                }
                // Imprime a lista de estudantes
                println!("{student_list}");
            }
            Ok(MainOption::Search) => {
                // Pesquisar por aluno. Só funciona se tiver alguém lá
                if student_list.is_empty() {
                    println!("Erro: Lista vazia");
                } else {
                    'choice : loop {
                        print!(
                            "Você quer consultar por:\n\
                        1 - Nome\n\
                        2 - Idade\n\
                        3 - Nota\n\
                        * - Voltar ao menu principal\n\
                        Selecione uma opção: ");
                        read_line!(input);
                        match input.trim().parse::<SearchOption>() {
                            Ok(SearchOption::Name) => {
                                // Nome
                                'data : loop {
                                    print!("Digite o nome do aluno pra procurar: ");
                                    read_line!(input);

                                    let result = Name::try_from_str(input.trim());

                                    match result {
                                        Ok(result) => {
                                            match student_list.get_by_name(result) {
                                                Some(student) => {
                                                    // Imprime o usuário
                                                    println!("Usuário encontrado:");
                                                    println!("{}", student);
                                                    println!("Aperte enter para continuar\x1b[K");
                                                    read_line!(input);
                                                    break 'data;
                                                }
                                                _ => {
                                                    println!("Nenhum aluno encontrado");
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            println!("Nome de aluno inválido");
                                        }
                                    }
                                }
                            },
                            Ok(SearchOption::Age) => {
                                // Idade
                                'data : loop {
                                    print!("Digite a idade pra procurar: ");
                                    read_line!(input);

                                    let result = input.trim().parse::<Age>();

                                    match result {
                                        Ok(res) => {
                                            match student_list.get_by_age(res) {
                                                Some(student) => {
                                                    // Imprime o usuário
                                                    println!("Usuário encontrado:");
                                                    println!("{}", student);
                                                    println!("Aperte enter para continuar\x1b[K");
                                                    read_line!(input);
                                                    break 'data;
                                                }
                                                _ => {
                                                    println!("Nenhum aluno encontrado");
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            println!("Idade inválida");
                                        }
                                    }
                                }
                            }
                            Ok(SearchOption::Grade) => {
                                // Nota
                                'data : loop {
                                    print!("Digite a nota pra procurar: ");
                                    read_line!(input);

                                    let result = input.trim().parse::<Grade>();

                                    match result {
                                        Ok(res) => {
                                            match student_list.get_by_grade(res) {
                                                Some(student) => {
                                                    // Imprime o usuário
                                                    println!("Usuário encontrado:");
                                                    println!("{}", student);
                                                    println!("Aperte enter para continuar\x1b[K");
                                                    read_line!(input);
                                                    break 'data;
                                                }
                                                _ => {
                                                    println!("Nenhum aluno encontrado");
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            println!("Nota inválida");
                                        }
                                    }
                                }
                            }
                            Err(_) => {
                                println!("Opção inválida");
                                break 'choice;
                            }
                        }
                    }
                }
            }
            Ok(MainOption::Remove) => {
                // Remover alguém
                if student_list.is_empty() {
                    println!("Erro: Lista vazia");
                } else {
                    'choice : loop {
                        print!(
                            "Você quer achar o usuário por:\n\
                        1 - Nome\n\
                        2 - Idade\n\
                        3 - Nota\n\
                        * - Voltar ao menu principal\n\
                        Selecione uma opção: ");
                        read_line!(input);
                        match input.trim().parse::<SearchOption>() {
                            Ok(SearchOption::Name) => {
                                // Nome
                                'data : loop {
                                    print!("Digite o nome do aluno pra procurar: ");
                                    read_line!(input);

                                    let result = Name::try_from_str(input.trim());

                                    match result {
                                        Ok(result) => {
                                            match student_list.get_by_name(result) {
                                                Some(student) => {
                                                    // Imprime o usuário
                                                    println!("Usuário encontrado:");
                                                    println!("{}", student);
                                                    println!("Deseja excluir este usuário?[s/N]");
                                                    read_line!(input);
                                                    if input.to_lowercase().starts_with('s') {
                                                        match student_list.remove_by_name(student.name) {
                                                            Ok(_) => {
                                                                println!("Aluno removido com sucesso")
                                                            }
                                                            Err(e) => {
                                                                println!("Erro ao remover o aluno: {e}")
                                                            }
                                                        }
                                                    } else {
                                                        println!("Aluno não removido");
                                                    }
                                                    println!("Aperte enter para continuar\x1b[K");
                                                    read_line!(input);
                                                    break 'data;
                                                }
                                                _ => {
                                                    println!("Nenhum aluno encontrado");
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            println!("Nome de aluno inválido");
                                        }
                                    }
                                }
                            },
                            Ok(SearchOption::Age) => {
                                // Idade
                                'data : loop {
                                    print!("Digite a idade pra procurar: ");
                                    read_line!(input);

                                    let result = input.trim().parse::<Age>();

                                    match result {
                                        Ok(res) => {
                                            match student_list.get_by_age(res) {
                                                Some(student) => {
                                                    // Imprime o usuário
                                                    println!("Usuário encontrado:");
                                                    println!("{}", student);
                                                    println!("Deseja excluir este usuário?[s/N]");
                                                    read_line!(input);
                                                    if input.to_lowercase().starts_with('s') {
                                                        match student_list.remove_by_age(student.age) {
                                                            Ok(_) => {
                                                                println!("Aluno removido com sucesso")
                                                            }
                                                            Err(e) => {
                                                                println!("Erro ao remover o aluno: {e}")
                                                            }
                                                        }
                                                    } else {
                                                        println!("Aluno não removido");
                                                    }
                                                    println!("Aperte enter para continuar\x1b[K");
                                                    read_line!(input);
                                                    break 'data;
                                                }
                                                _ => {
                                                    println!("Nenhum aluno encontrado");
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            println!("Idade inválida");
                                        }
                                    }
                                }
                            }
                            Ok(SearchOption::Grade) => {
                                // Nota
                                'data : loop {
                                    print!("Digite a nota pra procurar: ");
                                    read_line!(input);

                                    let result = input.trim().parse::<Grade>();

                                    match result {
                                        Ok(res) => {
                                            match student_list.get_by_grade(res) {
                                                Some(student) => {
                                                    // Imprime o usuário
                                                    println!("Usuário encontrado:");
                                                    println!("{}", student);
                                                    println!("Deseja excluir este usuário?[s/N]");
                                                    read_line!(input);
                                                    if input.to_lowercase().starts_with('s') {
                                                        match student_list.remove_by_grade(student.grade) {
                                                            Ok(_) => {
                                                                println!("Aluno removido com sucesso")
                                                            }
                                                            Err(e) => {
                                                                println!("Erro ao remover o aluno: {e}")
                                                            }
                                                        }
                                                    } else {
                                                        println!("Aluno não removido");
                                                    }
                                                    println!("Aperte enter para continuar\x1b[K");
                                                    read_line!(input);
                                                    break 'data;
                                                }
                                                _ => {
                                                    println!("Nenhum aluno encontrado");
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            println!("Nota inválida");
                                        }
                                    }
                                }
                            }
                            Err(_) => {
                                println!("Opção inválida");
                                break 'choice;
                            }
                        }
                    }
                }
            }
            Ok(MainOption::Clean) => {
                student_list.clear();
                println!("Lista limpa com sucesso");
            }
            Ok(MainOption::Destroy) => {
                if !student_list.is_empty() {
                    println!("Lista não está vazia");
                } else {
                    student_list = StudentList::new();
                    println!("Lista destruída com sucesso");
                }
            }
            Ok(MainOption::Exit) => {
                break 'main;
            }
            Err(_) => {
                println!("Por favor insira um número válido");
            }
        }
        println!("Aperte enter para continuar\x1b[K");
        read_line!(input);
    }
}
