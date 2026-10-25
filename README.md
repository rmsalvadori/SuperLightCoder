# Super Light Coder ⚡

Um editor de código e texto **extremamente leve**, rápido e responsivo, desenvolvido em **Rust**, no estilo Notepad clássico moderno, com foco em eficiência, baixo consumo de memória e inicialização instantânea.

---

## 🚀 Características Principais

- **Ultra Leve & Portátil**: Executável único compilado nativamente em Rust (~3.5 MB), com motor gráfico moderno WGPU (DirectX 12 / Vulkan nativo no Windows) e VSync ativo, livre de flickering ou tearing, consumindo pouquíssima memória RAM e com inicialização instantânea.
- **Reabertura Inteligente**: Ao abrir o Super Light Coder sem argumentos, ele restaura e reabre automaticamente o último arquivo trabalhado.
- **Ícone Nativo Exclusivo ("O Raio Conectado")**: Ícone embutido no `.exe`, no instalador e na barra de tarefas, com formato de raio estilizado cujas pontas superior e inferior simulam os caracteres `<` e `>`.
- **Navegador de Arquivos Lateral (Explorer)**:
  - 📁 Ícone de pasta na barra de status (com destaque visual azul quando ativo) e atalho `Ctrl + B`.
  - Exibe a árvore e lista de arquivos da pasta aberta com ícones representativos por tipo (`.rs`, `.py`, `.sql`, `.js`, `.md`, `.env`, `.htaccess`, `.gitignore`, `.cmd`, `.sh`, `.ps1`, etc.).
  - Suporte completo a arquivos de configuração dotfiles (`.gitignore`, `.htaccess`, `.env`, etc.) sem ocultação acidental.
  - Ao alternar entre arquivos modificados, pergunta de forma inteligente se deseja salvar as alterações antes de prosseguir.
  - Caso nenhum arquivo esteja aberto, exibe um botão amigável para abrir uma pasta.
- **Edição em Coluna / Multi-Cursor (Estilo VSCode / Sublime)**:
  - `Alt + Botão Esquerdo`: Arraste vertical para criar múltiplos cursores alinhados em coluna.
  - `Ctrl + Alt + Botão Esquerdo`: Adiciona novos cursores em múltiplas posições ou colunas simultâneas.
  - Digitação, colagem e exclusão simultânea em todas as linhas selecionadas.
- **Tipografia e Visual de Alta Qualidade**:
  - Fonte **JetBrains Mono** embutida nativamente no binário (garante renderização nítida de código em qualquer computador sem precisar instalar fontes no Windows).
- **Desfazer e Refazer Ilimitados**:
  - Sistema robusto de histórico de edições com `Ctrl + Z` e `Ctrl + Y`.
- **Arrastar e Soltar (Drag & Drop)**:
  - Arraste qualquer arquivo ou pasta do Windows Explorer diretamente para o editor para abrir instantaneamente.
- **Barra de Informações Inferior**:
  - 📁 **Explorador**, ⚙ **Configurações**, e botões rápidos ao lado:
    - 📄 **Novo Arquivo** (`Ctrl + N`)
    - 📂 **Abrir Arquivo** (`Ctrl + O`)
    - 💾 **Salvar Arquivo** (`Ctrl + S`, com destaque visual dinâmico quando houver alterações pendentes)
  - 🪄 **Botão de Formatação Rápida `{ }`**.
  - ☑ **Quebrar linha**: Checkbox para alternar a quebra automática de linha visual (Soft Wrap) instantaneamente.
  - Posição do cursor: `Ln X, Col Y`.
  - Contagem total de caracteres e seleção (`N caracteres` / `N selecionados`).
  - Seletor rápido de Linguagem (auto-detectado por extensão ou selecionável manualmente).
  - Indicador de Zoom (`100%`).
  - Tipo de quebra de linha: `Unix (LF)` ou `Windows (CRLF)` com alternância em 1 clique.
  - Codificação: `UTF-8`.
- **Gutter com Numeração de Linhas Otimizado**:
  - Largura compacta e proporcional aos dígitos.
  - Posicionamento intuitivo entre o explorador e o código.
  - Destaque sutil da linha ativa do cursor.
- **Integração com o Menu de Contexto do Windows**:
  - Opção nas Configurações para adicionar ou remover com um clique **"Editar no Super Light Coder"** posicionado no topo do menu de contexto de qualquer arquivo no Windows Explorer (registrado em `HKCU\Software\Classes\*\shell` com `Position="Top"`, sem necessidade de privilégios de Administrador).
- **Temas**:
  - 🌙 **Tema Escuro (Dark)** (moderno e confortável para os olhos).
  - ☀️ **Tema Claro (Light)** (clássico e limpo estilo bloco de notas).
- **Realce de Sintaxe para Mais de 25 Linguagens e Formatos**:
  - Rust, SQL, Python, JavaScript, TypeScript, HTML, PHP, Java, C, C#, C++, Go, Pascal, Perl, Lua, Ruby, XML, JSON, SVG, TOML e texto puro (`.txt`).
  - **Suporte Completo a Scripts e Configurações**:
    - 🔑 **Environment (`.env`, `.env.*`)**: Destaque para variáveis `CHAVE=VALOR`, `export`, strings, números, booleanos, interpolações `${VAR}` e comentários `#`.
    - 📝 **Markdown (`.md`)**: Destaque para títulos `#`, listas, links `[texto](url)`, citações `>` e trechos de código embutidos com crases ``` `code` ```.
    - 🔒 **Apache (`.htaccess`)**: Destaque para diretivas (`RewriteRule`, `RewriteCond`, `AuthType`, `Options`), tags `<IfModule>`, variáveis `%{...}` e flags `[NC,L]`.
    - 🐙 **Git (`.gitignore`, `.gitattributes`)**: Destaque para comentários `#`, negações `!`, wildcards `*` e separadores de caminho `/`.
    - ⚡ **Batch / Windows CMD (`.cmd`, `.bat`)**: Comentários `REM` / `::`, comandos internos (`echo`, `set`, `goto`, `call`), variáveis `%VAR%` / `!VAR!` e labels `:label`.
    - 🐚 **Shell Script (`.sh`, `.bash`, `.zsh`)**: Shebangs, comentários `#`, comandos, variáveis de ambiente `$VAR` e estruturas de controle.
    - 💠 **PowerShell (`.ps1`, `.psm1`, `.psd1`)**: Comentários `#` e `<# #>`, cmdlets (`Get-*`, `Set-*`, `Write-*`), parâmetros `-Name` e variáveis `$var`.
- **Formatadores de Código Embutidos (Zero Dependências Externas)**:
  - **SQL**: Quebra consultas de linha única em múltiplas linhas identadas com destaque para palavras-chave (`SELECT`, `FROM`, `WHERE`, `JOIN`, `GROUP BY`, `ORDER BY`, `LIMIT`, etc.).
  - **JSON**: Formatação e identação em árvore de JSON comprimido.
  - **HTML / XML / SVG**: Identação hierárquica por tags e quebra de linhas automática.
  - **C, C++, C#, Java, JS, TS, Rust, PHP, Go**: Identação inteligente por chaves `{ ... }` e blocos.
  - **Python, Pascal, Lua, Ruby, Markdown**: Formatadores dedicados para cada estrutura sintática.

---

## ⌨ Atalhos do Teclado

| Atalho | Função |
| :--- | :--- |
| `Ctrl + N` | Novo Arquivo |
| `Ctrl + O` | Abrir Arquivo |
| `Ctrl + S` | Salvar Arquivo |
| `Ctrl + Shift + S` | Salvar Como... |
| `Ctrl + B` | Abrir / Fechar Explorador de Arquivos (📁) |
| `Ctrl + F` | **Formatar Código Automaticamente** |
| `Ctrl + ,` | Abrir / Fechar Configurações (⚙) |
| `Alt + Arrastar` | Seleção em Coluna / Multi-Cursor |
| `Ctrl + Alt + Arrastar` | Adicionar Seleção em Coluna aos cursores existentes |
| `Ctrl + +` ou `Ctrl + =` | Aumentar Zoom |
| `Ctrl + -` | Diminuir Zoom |
| `Ctrl + 0` | Resetar Zoom (100%) |
| `Ctrl + Z` | Desfazer (Undo) |
| `Ctrl + Y` | Refazer (Redo) |
| `Ctrl + A` | Selecionar Tudo |
| `Ctrl + C` / `Ctrl + V` / `Ctrl + X` | Copiar, Colar e Recortar |

---

## 📦 Distribuição & Instalação (Release v0.1)

O **Super Light Coder** é distribuído para **Windows x64** em dois formatos:

1. **Instalador Oficial (`Setup.exe`)**:
   - Cria o atalho no **Menu Iniciar** do Windows e opção de atalho na Área de Trabalho.
   - Instalação limpa e rápida por usuário (sem exigir elevação UAC/Administrador).
   - Inclui desinstalador no Painel de Controle / Configurações do Windows.
   - Arquivo: `dist/SuperLightCoder-v0.1.0-Setup-x64.exe`

2. **Versão Portátil (`.zip`)**:
   - Não requer instalação. Basta extrair e executar `slc.exe`.
   - Pode ser transportado em pendrive ou executado em qualquer pasta.
   - Arquivo: `dist/SuperLightCoder-v0.1.0-Portable-x64.zip`

---

## 🛠 Como Executar e Compilar

### Execução via Linha de Comando:
```powershell
slc.exe "caminho\para\arquivo.sql"
```

### Compilar a partir do código-fonte:
Requer o compilador Rust instalado ([rustup.rs](https://rustup.rs/)).
```powershell
cargo build --release
```
O executável final é gerado em `target\release\SuperLightCoder.exe`.
