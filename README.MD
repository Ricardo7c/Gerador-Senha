# Gerador de Senhas Determinístico

O **Gerador de Senhas** é um utilitário desktop leve e seguro que transforma frases fáceis de lembrar em senhas fortes e complexas usando criptografia determinística (SHA-256).

> **Conceito Determinístico:** O aplicativo não salva suas senhas em nenhum banco de dados ou nuvem. Sempre que você inserir a mesma **Frase**, **Pepper** e configurações, a mesma senha será gerada.

---

### Requisitos e Instalação

O software é distribuído como um único arquivo executável portátil (não requer instalação):

* **Linux:** Execute o arquivo `gerador_senha` via terminal (`./gerador_senha`) ou com duplo clique.
* **Windows:** Execute o arquivo `gerador_senha.exe` com duplo clique.

---

### Interface e Campos de Entrada

| Campo | Obrigatório? | Descrição | Exemplo de Uso |
| --- | --- | --- | --- |
| **Frase** | Sim | Frase-chave ou texto base memorizado. | `meu cafe favorito de manha` |
| **Pepper** | Não | Segredo extra (salt) mascarado para elevar a segurança. Pode ser usado como identificador de serviço. | `netflix`, `banco_xyz` ou `senha_mestra` |
| **Tamanho** | Sim | Comprimento final da senha gerada (entre **4** e **128** caracteres). | Padrão: `16` |
| **Incluir símbolos** | Sim (Checkbox) | Define se a senha incluirá caracteres especiais (`!@#$%^&*()-_+=`). | Marcado por padrão |

---

### Passo a Passo de Uso

1. **Digite a Frase Base:** No campo **Frase**, insira o texto que você usará como base para sua senha.
2. **Adicione um Pepper (Opcional):** No campo **Pepper**, adicione uma palavra-chave extra ou o nome do serviço onde a senha será utilizada.
3. **Ajuste o Tamanho e Símbolos:** Arraste ou clique no campo **Tamanho** para definir o número de caracteres e marque/desmarque a opção **Incluir símbolos**.
4. **Gere a Senha:** Clique no botão **🔑 GERAR SENHA**.
5. **Copie o Resultado:** A senha aparecerá centralizada na parte inferior da tela. Clique no botão com o ícone **📋** para copiar o texto diretamente para a área de transferência.

---

### Dicas de Segurança e Boas Práticas

* **Reutilização Inteligente:** Em vez de usar senhas repetidas em sites diferentes, use a mesma frase-base e troque apenas o campo **Pepper** com o nome do site (ex.: `google`, `github`, `email`). Isso criará senhas completamente diferentes para cada serviço sem a necessidade de memorizar dezenas de senhas aleatórias.
* **Complexidade Mínima Garantida:** Toda senha gerada com 4 ou mais caracteres contém obrigatoriamente letras maiúsculas, minúsculas, números e (se habilitado) símbolos.
* **Privacidade Total:** A aplicação opera 100% offline e não realiza conexões de rede nem telemetria.