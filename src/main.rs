#![allow(dead_code)]
#![allow(unused_variables)]
/*
    X Crie uma coleção
    X Insira elementos na coleção
    X Liste os elementos da coleção
    X Consulte um dos elementos da coleção
    X Remova um elemento da função
    Esvazie a coleção

    typedef struct {
        char nome[30];
        int idade;
        float nota;
    } ALUNO;
 */
use std::io;
use crate::student::StudentList;

mod student;

fn main() {
    let mut input : String = String::new();
    let mut student_list :StudentList = StudentList::new();

    'main: loop {
        println!("{}", student_list.status());
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
        input.clear();
        io::stdin().read_line(&mut input).expect("Falha ao ler a string");

        match input.trim().parse::<u8>() {
            Ok(1) => {
                // Criar coleção
                // Pergunta a quantidade de caracteres
                'quant: loop {
                    println!("Digite a quantidade de itens para pré-alocar");
                    input.clear();
                    io::stdin().read_line(&mut input).expect("Falha ao ler a string");
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
