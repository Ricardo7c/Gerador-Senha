# Gerador de Senhas Determinístico

Aplicação desktop leve e multiplataforma desenvolvida em **Rust**, utilizando o framework gráfico **`eframe` / `egui**` e funções criptográficas de hash via **`sha2`**.

O objetivo da aplicação é gerar senhas determinísticas a partir de uma frase-passe (e um salt/pepper opcional), garantindo que a mesma combinação de entradas sempre resulte na mesma senha de saída, sem necessidade de armazenamento local ou em nuvem.

---

## 1. Dependências do Projeto (`Cargo.toml`)

Para compilar o código, declare as seguintes dependências no arquivo de manifesto:

```toml
[dependencies]
eframe = { version = "0.36.1", default-features = false, features = ["default_fonts", "glow", "wayland", "x11"] }
sha2 = "0.11.0"

```

---

## 2. Arquitetura e Estrutura do Código

O projeto está dividido em quatro seções principais:

### 2.1. Lógica Criptográfica (`phrase_to_password`)

* **Assinatura:**
```rust
fn phrase_to_password(
    phrase: &str,
    length: usize,
    include_symbols: bool,
    salt: &str,
) -> Result<String, String>

```


* **Validação:** Exige tamanho mínimo de 4 caracteres (necessário para acomodar ao menos uma classe de cada tipo de caractere).
* **Processo:**
1. Concatena os conjuntos de caracteres permitidos (minúsculas, maiúsculas, dígitos e, opcionalmente, símbolos).
2. Gera um hash **SHA-256** derivado de `salt + phrase`.
3. Garante **complexidade mínima** inserindo deterministicamente pelo menos 1 caractere de cada classe obrigatória.
4. Preenche o restante do comprimento solicitado consumindo bytes sequenciais do digest SHA-256 via módulo (`%`).
5. Executa uma **rotação determinística** no vetor de saída com base em um byte do hash para evitar que as classes obrigatórias fiquem previsivelmente nos primeiros índices.



---

### 2.2. Estado da Aplicação (`GeradorSenha`)

Armazena os dados reativos da interface durante a execução:

| Campo | Tipo | Descrição |
| --- | --- | --- |
| `frase` | `String` | Texto base inserido pelo usuário. |
| `pepper` | `String` | Salt/segredo adicional mascarado (`password(true)`). |
| `senha` | `String` | Resultado da senha gerada. |
| `tamanho` | `usize` | Comprimento desejado da senha (padrão: 16, intervalo: 4 a 128). |
| `incluir_simbolos` | `bool` | Flag para inclusão de caracteres especiais (`!@#$%^&*()-_+=`). |
| `mensagem` | `String` | Feedback de status (ex: "Senha copiada!") ou mensagens de erro. |

---

### 2.3. Interface Gráfica (`impl eframe::App`)

Implementada no método `ui`:

* **Layout vertical centralizado:** Mantém o painel coeso no centro da janela.
* **Entradas de dados:**
* Linhas com `egui::TextEdit` para **Frase** e **Pepper** (oculto).
* Linha de controle com `egui::DragValue` para o **Tamanho**, checkbox para **Símbolos** e botão de ação **🔑 GERAR SENHA**.


* **Exibição do Resultado:**
* Renderizado condicionalmente apenas quando `!self.senha.is_empty()`.
* **Centralização Dinâmica:** Calcula a largura ocupada (`monospace` + botão `📋`) e aplica um deslocamento proporcional via `ui.add_space` baseado no `ui.available_width()`.
* **Área de Transferência:** O botão de cópia aciona `ui.ctx().copy_text(...)` para transferir a senha para a área de transferência do sistema operacional.



---

### 2.4. Inicialização (`main`)

Configura a janela nativa através de `eframe::NativeOptions` e `egui::ViewportBuilder`:

* **Dimensões iniciais:** 370 x 150 pixels.
* **Redimensionamento:** Desativado (`with_resizable(false)`).

---

## 3. Guia de Compilação e Execução

Para compilar e executar o projeto em modo de desenvolvimento:

```bash
cargo run

```

Para gerar o binário otimizado para produção:

```bash
cargo build --release

```