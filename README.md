# TAD COLLECTION GENÉRICO - Reimplementado em [RUST](https://www.rust-lang.org)
Trabalho original disponível em [colecoes_ed](https://github.com/uotlaf/colecoes_ed/)

### Descrição
Implemente o TAD Collection Genérico e o utilize para fazer um programa com os seguintes requisitos(obrigatório utilizar a mesma API):

### Objetivos
- [X] Defina uma [estrutura do TIPO ESCOLHIDO](src/student.rs)(Aluno, Tipo, Time, etc) que tenha pelo menos os seguintes campos:
  - [X] Um campo char[30]
  - [X] Um campo inteiro
  - [X] Um campo float
- [ ] Implemente as seguintes operações:
  - [X] Criar uma coleção
  - [X] Inserir um TIPO ESCOLHIDO na coleção
  - [X] Remover um TIPO ESCOLHIDO na coleção identificado pelo campo char, int e float
  - [X] Consultar um TIPO ESCOLHIDO na coleção identificado pelo campo char, int e float
  - [X] Listar os elementos na coleção
  - [X] Destruir a coleção se estiver vazia
  - [X] Esvaziar a coleção
- [X] Não imprima nada dentro de uma função que não seja a main()

### Roteiro para teste
1 - Crie uma coleção \
2 - Insira três elementos na coleção \
3 - Liste os elementos da coleção \
4 - Consulte um dos elementos da coleção \ 
5 - Remova o segundo elemento inserido na coleção \
6 - Liste os elementos da coleção \
7 - Esvazie a coleção \
8 - Liste os elementos da coleção

### Como construir
```sh 
cargo build --release
# Binário disponível em target/release/colecoes_ed_rust
```