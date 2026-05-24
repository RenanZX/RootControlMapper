# Comandos

Existem 8 comandos disponeis no root-ctrl-mapper:
- `-v` - Print Version
- `-b` : Abre o app em background
- `-k` : Mata a instancia do app em background
- `-j` : Consulta o caminho dos arquivos JSON contendo os botoes/teclas mapeadas
- `-s` : Consulta o caminho dos scripts
- `-py` : Consulta a raiz da Instalação Python do ambiente do Root Control Mapper
- `-pyi` : Consulta o ativador do ambiente Python do Root Control Mapper
- `-h` : Consulta este documento helper disponivel no git
- `-hc` : Abre o helper de linha de comandos CLI

> Você pode consultar todos os comandos disponíveis digitando `root-ctrl-mapper -hc`

# Sobre as Configurações

O root control mapper já vem pré-configurado para uso, acompanhado de dois scripts uteis para melhorar sua experiência de gameplay.

# Scripts Personalizados
- `pp-browser`: Caso queira jogar enquanto ouve uma música, um podcast ou assistir um vídeo no navegador de internet enquanto joga com o emulador/jogo mutado, esse script permite que você alterne entre pausar um video/audio e mutar/desmutar um jogo.
> Esse script só é compatível com navegador Brave

- `record_obs`: Um script que grava gameplays do obs em background, sem a necessidade abri-lo manualmente, caso queira gravar uma gameplay enquanto joga.
> - Necessário ativar a opção web socket do seu obs studio: `OBS Studio > Ferramentas > Configurações de WebSocket > Ativar WebSocket`
> - Caso deseje uma experiência mais próxima do console, ative a opção `Minimizar para a bandeja do sistema` nas suas configurações do seu obs studio.

Você pode personalizar seus próprios scripts colocando ou o caminho do script completo ou adicionando o script dentro da pasta de `scripts` da raiz de instalação do `root-ctrl-mapper`, basta navegar ate ela

```shell
cd $(root-ctrl-mapper -s)
```

## Instalando libs do Python no ambiente do Root Control Mapper

O app utiliza um ambiente isolado(venv) por questões de segurança, para acessar o ambiente python do `Root Control Mapper`, digite:

```shell
source $(root-ctrl-mapper -pyi)
```

Instale todas as depêndencias que precise usando o comando `pip install`, e saia do ambiente com o comando:
```shell
deactivate
```

# Modos de Operação

O root control mapper possui 2 modos de operação intercambeáveis:

- `Game Mode`: Modo para utilizar na sua jogatina, você pode configurar scripts para tirar prints, gravar gameplays, executar macros personalizadas ou fazer o que você quiser enquanto joga seus jogos na sua máquina Linux.
- `Mouse Mode`: Modo para utilizar ao mexer no seu computador, mover o mouse ou digitar no teclado usando um `teclado virtual` com uma interface amigável ao controle que o root control mapper oferece.

> Ao iniciar o app, o modo padrão é o `Game Mode`, para alterar é só apertar a sequência de alterar modo

# Personalizando Comandos

Abra o diretorio `json` na raiz da instalação do programa, lá terá dois arquivos `.json` denominados `game_mode.json` e `mouse_mode.json`, ambos arquivos representam cada modo de operação respectivamente.

A lista a seguir tem todos os botoes/macros mapeaveis para personalizar sua configuração.


# Botões mapeaveis (Controle)

- BTN_SELECT
- BTN_START
- BTN_HOME ou BTN_MODE
- BTN_RECORD
- BTN_LB
- BTN_RB
- BTN_LT
- BTN_RT
- BTN_LTHUMB
- BTN_RTHUMB
- BTN_A
- BTN_B
- BTN_X
- BTN_Y
- DPAD_LEFT
- DPAD_UP
- DPAD_DOWN
- DPAD_RIGHT

> Analogicos (LEFT_STICK/RIGHT_STICK) so podem ser mapeados no ponteiro do mouse

# Teclas/Botoes Mapeaveis (Mouse e Teclado)

- ALT
- KEY_LEFTALT
- KEY_RIGHTALT
- CTRL
- KEY_LEFTCTRL
- KEY_RIGHTCTRL
- SUPER 
- KEY_LEFTSUPER
- KEY_LEFTMETA
- KEY_RIGHTSUPER
- KEY_RIGHTMETA
- SHIFT
- KEY_LEFTSHIFT
- KEY_RIGHTSHIFT
- BACKSPACE
- ENTER
- RETURN
- ESC
- ESCAPE
- SPACE
- MOUSE_RIGHTCLICK
- MOUSE_LEFTCLICK
- ARROW_RIGHT
- ARROW_LEFT
- ARROW_UP
- ARROW_DOWN
- SCROLL_DOWN
- SCROLL_UP
- BACKSPACE
- INSERT
- INS
- DELETE
- DEL
- HOME
- END
- PAGEUP
- PGUP
- PAGEDOWN
- PGDN
- MINUS
- EQUAL
- LEFTBRACE
- RIGHTBRACE
- SEMICOLON
- APOSTROPHE
- GRAVE
- BACKSLASH
- COMMA
- DOT
- SLASH
- KP0
- KP1
- KP2
- KP3
- KP4
- KP5
- KP6
- KP7
- KP8
- KP9
- KPASTERISK
- KP_MULTIPLY
- KPMINUS
- KP_SUBTRACT
- KPPLUS
- KP_ADD
- KPDOT
- KP_DEL
- KPENTER
- KPSLASH
- KP_DIVIDE
- PRINTSCREEN
- PRINT
- PAUSE
- MUTE
- VOLUME_MUTE
- VOLUMEDOWN
- VOLUMEUP
- PLAYPAUSE
- STOP
- NEXTSONG
- NEXT
- PREVIOUSSONG
- PREV
- KEY_A
- KEY_B
- KEY_C
- KEY_D
- KEY_E
- KEY_F
- KEY_G
- KEY_H
- KEY_I
- KEY_J
- KEY_K
- KEY_L
- KEY_M
- KEY_N
- KEY_O
- KEY_P
- KEY_Q
- KEY_R
- KEY_S
- KEY_T
- KEY_U
- KEY_V
- KEY_W
- KEY_X
- KEY_Y
- KEY_Z
- KEY_0
- KEY_1
- KEY_2
- KEY_3
- KEY_4
- KEY_5
- KEY_6
- KEY_7
- KEY_8
- KEY_9
- KEY_F1
- KEY_F2
- KEY_F3
- KEY_F4
- KEY_F5
- KEY_F6
- KEY_F7
- KEY_F8
- KEY_F9
- KEY_F10
- KEY_F11
- KEY_F12


> O botão record no momento apenas foi testado e validado somente no controle `Gamesir Nova Lite`

## Botoes

- `buttons`: A sequência de um ou mais botões do seu controle Xbox a ser mapeado

## Macro

- `macro_keys`: Representa a combinação de uma ou mais teclas do teclado para serem executadas por uma sequência definida em `buttons`

## Script

- `exec`: Caminho para um script bash ou o nome do script caso esteja na pasta `scripts`
- `py_exec`: Caminho para um script python ou o nome do script caso esteja na pasta `scripts`

## Flags Especiais (json)

- `change_mode`: Muda o modo de operação do controle para o mouse/jogo dependendo da configuração do arquivo `.json`
- `virtual_keyboard`: Ativa o teclado virtual (disponível apenas no `mouse mode`)
- `mouse_move`: Move o ponteiro do mouse (disponível apenas no `mouse mode` e só pode ser atribuído aos analogicos `LEFT_STICK/RIGHT_STICK`)
- `clipboard_buffer`: O buffer do clipboard (disponivel apenas no `mouse mode`) do teclado virtual(`virtual_keyboard`), você pode salvar um texto no buffer ou uma `macro` (combinção de teclas) a partir do teclado virtual. 

## Tipos de clique
- `double_click`: Recebe um clique duplo da entrada do controlador
- `long_press`: Recebe um clique pressionado por um longo periodo da entrada do controlador

# Como customizar suas configurações

- Abra a pasta `json` na raiz da sua instalação: 
```shell
cd $(root-ctrl-mapper -j)
```

- Você encontrará dois arquivos `game_mode.json` e `mouse_mode.json`

- Personalize sua configuração acordo com o template a seguir:

```json
[
    {"buttons": ["BTN_A"], "exec": "meu_script.sh"},
    {"buttons": ["BTN_RTHUMB"], "change_mode": true},
    {"buttons": ["BTN_HOME", "BTN_SELECT"], macro_keys:["KEY_F5"]},
    {"buttons": ["BTN_HOME"], "double_click": true, "py_exec": "meu_script.py"}
    ...
]

```

> O `Game Mode` é o modo que você irá usar durante suas jogatinas, é recomendável sempre colocar um combo de botões com `BTN_HOME` ou `BTN_SELECT` para que não afete sua experiência ao jogar.

# Como utilizar o teclado virtual

<p align="center">
    <img src="../assets/vk-template.png" width="600"/>
</p>

O teclado virtual do root ctrl mapper é um teclado criado para facilitar o uso do controle (joystick) ao digitar um texto ou usar um comando no Linux, a seguir está a legenda do que cada botão faz:

<p align="center">
    <img src="../assets/virtual-keyboard.png" width="850"/>
</p>


