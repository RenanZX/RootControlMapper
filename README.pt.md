<p align="center">
    <img src="assets/terminalroot-logo2.png" width="220"/>
</p>

# Root Control Mapper

Mapeador de joysticks para Linux desenvolvido em Rust. Permite o mapeamento de teclas e scripts personalizados (Bash/Python), contando com um um teclado virtual otimizado para controles.

<p align="center">
<a href="#uso">Uso</a> - <a href="#installation">Instalação</a> - <a href="readme/helper.pt.md#personalizando-comandos">Comandos</a> - <a href="readme/helper.pt.md#scripts-personalizados">Scripts</a> - <a href="dev/guide.pt.md">Desenvolvimento</a>
</p>

# 🎮 Use apenas seu Joystick sem mouse e sem teclado ✨

<p align="center">
<video controls muted src="https://github.com/user-attachments/assets/106b42c0-d611-489c-bdbb-9c703cccbb58" with="250px" height="250px"></video>
</p>

## Objetivos do projeto (atual e futuro)
- Criar uma experiência flexível e personalizada para uso do controle no Linux com todas as suas funcionalides, *sem limitações*.
- Corrigir possiveis bugs e adicionar mais teclas a medida que for necessário e o projeto evoluir.
- Adicionar compatibilidade com o botão share dos controles Xbox Series

## Instalação
1. Extraia o `.tar.gz`
2. Dê as permissões necessárias para o script e rode o comando para instalação:
```shell
chmod +x install.sh
./install.sh
```
3. Verifique a instalação com o comando:
```shell
root-ctrl-mapper -v
```
4. Caso queira consultar os comandos disponiveis:
```shell
root-ctrl-mapper -hc
```

## Uso

Para usar basta rodar: 

```shell
root-ctrl-mapper
``` 


## Comandos Pré-configurados

Não é necessário configurar nenhum comando, todos são pré-configurados na instalação, abaixo está a configuração padrão de cada Modo de operação respectivamente:

## Game Mode

<div style="text-align: center;">
    <img src="assets/game-mode.png" width="450"/>
</div>

> Nesse modo de operação, todos são combos de botões para não afetar sua experiência na jogatina

## Mouse Mode

<div style="text-align: center;">
    <img src="assets/mouse-mode.png" width="850"/>
</div>



- Para alterar os botoes, macros e scripts, consulte o [helper](readme/helper.pt.md) 
- Caso queira rodar em background sem o terminal, digite `root-ctrl-mapper -b` e caso queira fechar o app em background digite `root-ctrl-mapper -k`, todos os comandos estão disponíveis no [helper]("readme/helper.pt.md#comandos") deste repositório


<p align="center">
  <a href="https://www.buymeacoffee.com/renan_zx" target="_blank">
    <img src="https://img.shields.io/badge/Buy%20me%20a%20coffee-5F7FFF?style=for-the-badge&logo=buy-me-a-coffee&logoColor=FFDD00" alt="Buy Me a Coffee">
  </a>
</p>
