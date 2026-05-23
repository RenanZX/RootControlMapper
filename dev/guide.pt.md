# Contribuindo para o projeto

Caso queira contribuir com o aprimoramento do código:
1. Baixe o projeto no github, ou faça um fork deste repositório.
2. Nunca use a branch principal(`main`), crie uma branch com suas alterações e faça um `pull request` para revisão.
3. A nomenclatura de branchs seguem o padrão `snake_case`, você pode utilizar a ferramenta `git flow` para organizar seu fluxo de desenvolvimento.
4. Realize suas alterações seguindo os padrões estabelecidos nesse documento.

# Padrão/Estilo de código

Esse projeto adota as melhores práticas de codificação em rust, sendo:
- `snake_case`: Para funções e variáveis
- `PascalCase`: Para structs

# Compilação e execução

### Instalando dependencias da linguagem

Primeiramente instale as ferramentas necessárias para compilação de código em rust:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

- Pressione 1 e dê um enter para instalar a ferramenta
- Em seguida atualize seu ambiente:

```shell
source "$HOME/.cargo/env"
```

- Para verificar a instalação, apenas digite o comando a seguir no terminal:
```shell
rustc --version
```

Você pode saber mais na página [oficial da linguagem]("https://rust-lang.org/tools/install/")

### Preparando o ambiente

Na raiz do projeto, rode o seguinte comando no terminal:

```shell
./build.sh --debug
```

Isso irá criar todo o ambiente necessário para fazer testes, caso queira testar macros ou scripts que você desenvolveu, utilize a pasta `environment/debug` para fazer suas alterações.

### Rodando o projeto

Digite no terminal o comando padrão para executar apps em desenvolvimento no rust:
```shell
cargo run
```

### Compilando o projeto

- Na raiz do projeto, execute o script no terminal:

```shell
./build.sh --release
``` 

- Será criada uma pasta `build` com o executável do projeto e o script de instalação.
- Para criar o `.tar` basta digitar `./build.sh --release tar` ou caso ja tenha feito a build, `./build.sh --tar`
- Caso queira limpar as builds digite `./build.sh --clear`
- Verifique os comandos com help, se precisar:

```shell
./build.sh --help
```

### Como verificar os logs e rodar o app no modo debug

##### Mostra logs de nível info e acima (info, warn, error)
`RUST_LOG=info cargo run`

##### Mostra tudo, incluindo debug e trace (nível mais detalhado)
`RUST_LOG=trace cargo run`

##### Mostra logs apenas do seu módulo principal
`RUST_LOG=debug cargo run`
