#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod logica;

use logica::phrase_to_password;

use iced::clipboard;
use iced::widget::{button, checkbox, column, container, row, text, text_input, slider};
use iced::{Center, Element, Length, Size, Task};
use iced::font::{self, Font};

struct GeradorSenha {
    frase: String,
    pepper: String,
    mostrar_pepper: bool,
    senha: String,
    tamanho: usize,
    incluir_simbolos: bool,
    mensagem: String,
}

impl Default for GeradorSenha {
    fn default() -> Self {
        Self {
            tamanho: 16,
            incluir_simbolos: true,
            frase: String::new(),
            pepper: String::new(),
            mostrar_pepper: false,
            senha: String::new(),
            mensagem: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    FraseAlterada(String),
    PepperAlterado(String),
    AlternarMostrarPepper,
    TamanhoAlterado(usize),
    AlternarSimbolos(bool),
    GerarSenha,
    CopiarSenha,
}

impl GeradorSenha {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FraseAlterada(valor) => {
                self.frase = valor;
                Task::none()
            }
            Message::PepperAlterado(valor) => {
                self.pepper = valor;
                Task::none()
            }
            Message::AlternarMostrarPepper => {
                self.mostrar_pepper = !self.mostrar_pepper;
                Task::none()
            }
            Message::TamanhoAlterado(valor) => {
                self.tamanho = valor;
                Task::none()
            }
            Message::AlternarSimbolos(valor) => {
                self.incluir_simbolos = valor;
                Task::none()
            }
            Message::GerarSenha => {
                match phrase_to_password(
                    &self.frase,
                    self.tamanho,
                    self.incluir_simbolos,
                    &self.pepper,
                ) {
                    Ok(pwd) => {
                        self.senha = pwd;
                        self.mensagem.clear();
                    }
                    Err(err) => {
                        self.senha.clear();
                        self.mensagem = err;
                    }
                }
                if self.frase.trim().is_empty(){
                    self.senha.clear();
                }
                Task::none()
            }
            Message::CopiarSenha => {
                self.mensagem = "Senha copiada!".to_string();
                clipboard::write(self.senha.clone())
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // Campo Frase
        let campo_frase = row![
            text("Frase:").width(60),
            text_input("Digite sua frase...", &self.frase)
                .on_input(Message::FraseAlterada)
                .width(Length::Fill)
        ]
        .spacing(10)
        .align_y(Center);

        // Campo Pepper
        let icone_olho = if self.mostrar_pepper { "🔒" } else { "👁" };
        let campo_pepper = row![
            text("Pepper:").width(60),
            text_input("Opcional", &self.pepper)
                .secure(!self.mostrar_pepper)
                .on_input(Message::PepperAlterado)
                .width(Length::Fill),
            button(text(icone_olho)).on_press(Message::AlternarMostrarPepper)
        ]
        .spacing(10)
        .align_y(Center);

        // Controles de Configuração
        let controles = row![
            text("Tamanho:"),
            text(format!("{:02}", self.tamanho)).font(Font {
                weight: font::Weight::Bold,
                ..Default::default()
            }),
            slider(
                4.0..=64.0,
                self.tamanho as f64,
                |valor| Message::TamanhoAlterado(valor as usize),
            )
            .width(128),
            checkbox(self.incluir_simbolos)
                .label("Símbolos")
                .on_toggle(Message::AlternarSimbolos),
            button(text("🔑 Gerar")).on_press(Message::GerarSenha)
        ]
        .spacing(6)
        .align_y(Center)
        .width(Length::Fill);

        // Layout Principal
        let mut conteudo = column![campo_frase, campo_pepper, controles]
            .spacing(12)
            .align_x(Center);

        // Exibição da Senha
        if !self.senha.is_empty() {
            let bloco_senha = row![
                text(&self.senha).size(16),
                button(text("📋")).on_press(Message::CopiarSenha)
            ]
            .spacing(10)
            .align_y(Center);

            conteudo = conteudo.push(bloco_senha);
        }

        // Mensagens de Status / Erro
        if !self.mensagem.is_empty() {
            conteudo = conteudo.push(text(&self.mensagem).size(13));
        }

        container(conteudo)
            .padding(16)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .into()
    }
}



fn main() -> iced::Result {
    iced::application(
        GeradorSenha::default,
        GeradorSenha::update,
        GeradorSenha::view,
    )
    .title("Gerador de Senhas")
    .window(iced::window::Settings {
        size: Size::new(460.0, 220.0),
        resizable: false,
        ..Default::default()
    })
    .run()
}