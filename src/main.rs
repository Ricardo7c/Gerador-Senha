#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use sha2::{Digest, Sha256};

// ============================================================
// GERADOR DE SENHA
// ============================================================

fn phrase_to_password(
    phrase: &str,
    length: usize,
    include_symbols: bool,
    salt: &str,
) -> Result<String, String> {
    if length < 4 {
        return Err("O tamanho mínimo da senha é 4 caracteres.".to_string());
    }

    let lowers = b"abcdefghijklmnopqrstuvwxyz";
    let uppers = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let digits = b"0123456789";
    let symbols = b"!@#$%^&*()-_+=";

    // --------------------------------------------------------
    // Cria a lista de caracteres possíveis
    // --------------------------------------------------------
    let mut combined = Vec::new();
    combined.extend_from_slice(lowers);
    combined.extend_from_slice(uppers);
    combined.extend_from_slice(digits);

    if include_symbols {
        combined.extend_from_slice(symbols);
    }

    // --------------------------------------------------------
    // SHA-256
    // --------------------------------------------------------
    let mut hasher = Sha256::new();
    hasher.update(salt.as_bytes());
    hasher.update(phrase.as_bytes());
    let digest = hasher.finalize();

    // --------------------------------------------------------
    // Classes obrigatórias
    // --------------------------------------------------------
    let classes: Vec<&[u8]> = if include_symbols {
        vec![lowers, uppers, digits, symbols]
    } else {
        vec![lowers, uppers, digits]
    };

    let mut pwd_chars = Vec::new();

    // --------------------------------------------------------
    // Garante pelo menos um caractere de cada classe
    // --------------------------------------------------------
    for (i, class) in classes.iter().enumerate() {
        let byte = digest[i];
        let index = byte as usize % class.len();
        pwd_chars.push(class[index]);
    }

    // --------------------------------------------------------
    // Preenche o restante da senha
    // --------------------------------------------------------
    let mut idx = classes.len();
    while pwd_chars.len() < length {
        let byte = digest[idx % digest.len()];
        let index = byte as usize % combined.len();
        pwd_chars.push(combined[index]);
        idx += 1;
    }

    // --------------------------------------------------------
    // Rotação determinística
    // --------------------------------------------------------
    let rot = digest[classes.len()] as usize % length;
    pwd_chars.rotate_left(rot);

    // --------------------------------------------------------
    // Converte Vec<u8> para String
    // --------------------------------------------------------
    Ok(String::from_utf8(pwd_chars).unwrap())
}

// ============================================================
// ESTADO DA APLICAÇÃO
// ============================================================

struct GeradorSenha {
    frase: String,
    pepper: String,
    mostrar_pepper: bool,
    senha: String,
    tamanho: usize,
    incluir_simbolos: bool,
    mensagem: String,
}

// ============================================================
// VALORES INICIAIS
// ============================================================

impl Default for GeradorSenha {
    fn default() -> Self {
        Self {
            frase: String::new(),
            pepper: String::new(),
            mostrar_pepper: false,
            senha: String::new(),
            tamanho: 16,
            incluir_simbolos: true,
            mensagem: String::new(),
        }
    }
}

// ============================================================
// INTERFACE EGUI (API 0.36+)
// ============================================================

impl eframe::App for GeradorSenha {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                // FRASE
                ui.horizontal(|ui| {
                    ui.label("Frase:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.frase)
                            .desired_width(320.0)
                            .hint_text("Digite sua frase..."),
                    );
                });

                ui.add_space(5.0);

                // PEPPER
                ui.horizontal(|ui| {
                    ui.label("Pepper:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.pepper)
                            .desired_width(285.0)
                            .password(!self.mostrar_pepper)
                            .hint_text("Opcional"),
                    );

                    let icon = if self.mostrar_pepper { "🔒" } else { "👁" };
                    let tooltip = if self.mostrar_pepper {
                        "Ocultar pepper"
                    } else {
                        "Exibir pepper"
                    };

                    if ui.button(icon).on_hover_text(tooltip).clicked() {
                        self.mostrar_pepper = !self.mostrar_pepper;
                    }
                });

                ui.add_space(10.0);

                // TAMANHO
                ui.horizontal(|ui| {
                    ui.label("Tamanho:");
                    ui.add(
                        egui::DragValue::new(&mut self.tamanho)
                            .range(4..=128),
                    );

                    ui.add_space(10.0);
                    // SÍMBOLOS
                    ui.checkbox(&mut self.incluir_simbolos, "Incluir símbolos");

                    // BOTÃO GERAR
                    if ui.button("🔑  GERAR SENHA").clicked() {
                        match phrase_to_password(
                            &self.frase,
                            self.tamanho,
                            self.incluir_simbolos,
                            &self.pepper,
                        ) {
                            Ok(password) => {
                                self.senha = password;
                                self.mensagem.clear();
                            }
                            Err(error) => {
                                self.senha.clear();
                                self.mensagem = error;
                            }
                        }
                    }
                });

                ui.add_space(10.0);
    
                // SENHA GERADA
                if !self.senha.is_empty() {
                    ui.horizontal(|ui| {
                        let estimated_width = (self.senha.len() as f32 * 8.0) + 32.0;
                        let left_padding = ((ui.available_width() - estimated_width) / 2.0).max(0.0);
                        
                        ui.add_space(left_padding);

                        ui.monospace(&self.senha);

                        if ui.button("📋").on_hover_text("Copiar senha").clicked() {
                            ui.ctx().copy_text(self.senha.clone());
                            self.mensagem = "Senha copiada!".to_string();
                        }
                    });
                }

                // MENSAGEM DE STATUS/ERRO
                if !self.mensagem.is_empty() {
                    ui.add_space(2.0);
                    ui.label(&self.mensagem);
                }
            });
        });
    }
}

// ============================================================
// MAIN
// ============================================================

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([380.0, 140.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Gerador de Senhas",
        options,
        Box::new(|_cc| Ok(Box::new(GeradorSenha::default()))),
    )
}