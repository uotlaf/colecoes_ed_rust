# TAD COLLECTION GENÉRICO - Reimplementado em [RUST](https://www.rust-lang.org)
Trabalho original disponível em [colecoes_ed](https://github.com/uotlaf/colecoes_ed/)

### Descrição
Implemente o TAD Collection Genérico e o utilize para fazer um programa com os seguintes requisitos(obrigatório utilizar a mesma API):

### Objetivos
- [ ] Defina uma estrutura do TIPO ESCOLHIDO(Aluno, Tipo, Time, etc) que tenha pelo menos os seguintes campos:
  - [ ] Um campo char[30]
  - [ ] Um campo inteiro
  - [ ] Um campo float
- Implemente as seguintes operações:
  - [ ] Criar uma coleção
  - [ ] Inserir um TIPO ESCOLHIDO na coleção
  - [ ] Remover um TIPO ESCOLHIDO na coleção identificado pelo campo char, int e float
  - [ ] Consultar um TIPO ESCOLHIDO na coleção identificado pelo campo char, int e float
  - [ ] Listar os elementos na coleção
  - [ ] Destruir a coleção se estiver vazia
  - [ ] Esvaziar a coleção

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