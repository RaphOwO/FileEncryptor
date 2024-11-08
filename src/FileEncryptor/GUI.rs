use iced::widget::{Column, Container, Row};
use iced::widget::{button, button::Status, container, Button, text, Text, TextInput, checkbox, scrollable, vertical_space};
use iced::{Alignment, Border, Color, Element, Length, Padding, Shadow, Theme, Vector};
use std::process;
use std::path::PathBuf;
use rfd::FileDialog;

use crate::FileEncryptor::backend::{self, Algorithm};

pub struct FileEncryptor {
    page: Page,
    passphase: Passpharse,
    command: Option<Command>,
    selected_file: Option<PathBuf>,
    method: Option<Algorithm>,
    show_pass: bool,
    message: Option<String>,
    content: String,
}

#[derive(Debug, Clone)]
pub struct Passpharse {passphase: String, confirm: String}

#[derive(Debug, Clone)]
pub enum Message {
    PasspharseSubmit,
    PassphaseEnter(String, String),
    ChangePage(Page),
    SelectFile,
    SelectMethod(Algorithm),
    ShowPass(bool),
    Back,
    Exit,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Command {Decrypt, Encrpyt, Read}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Page {Menu, Passpharse(Command), SelectFile(Command), SelectMethod, Process(Command), Read}

fn menu_page() -> Container<'static, Message> { //menu page
    let column = Column::new()
        .push(btn("Encrpyt", Message::ChangePage(Page::SelectFile(Command::Encrpyt))))
        .push(btn("Decrypt", Message::ChangePage(Page::SelectFile(Command::Decrypt))))
        .push(btn("Read", Message::ChangePage(Page::SelectFile(Command::Read))))
        .padding(Padding::from([40, 20]))
        .align_x(Alignment::Center)
        .spacing(30);

    container(column)
        .padding(Padding::from(20))
        .style(|_theme: &Theme| {apperance()})
}

fn select_file(command: Command, path: Option<PathBuf>) -> Container<'static, Message> { //select file page
    let column = if let Some(file) = &path {
        Column::new()
        .push(text("Please select a file:").align_x(Alignment::Start).size(20))
        .push(btn("Select", Message::SelectFile))
        .push(text(format!("Selected File: {:?}", file)))
        .push(btn("Submit", Message::ChangePage(match command {
            Command::Encrpyt => Page::SelectMethod,
            _ => Page::Passpharse(command)
        })))
        .padding(Padding::from([30, 20]))
        .align_x(Alignment::Center)
        .spacing(30)
    } else {
        Column::new()
        .push(text("Please select a file:").align_x(Alignment::Start).size(20))
        .push(btn("Select", Message::SelectFile))
        .padding(Padding::from([30, 20]))
        .align_x(Alignment::Center)
        .spacing(30)
    };

    container(column)
        .width(Length::Fixed(500.0))
        .padding(Padding::from(20))
        .style(|_theme: &Theme| {apperance()})
}

fn select_method() -> Container<'static, Message> { //select method page
    let column = Column::new()
        .push(text("Please select an encryption method:").align_x(Alignment::Start).size(20))
        .push(btn("AesGcm", Message::SelectMethod(Algorithm::AesGcm)))
        .push(btn("AesGcmSiv", Message::SelectMethod(Algorithm::AesGcmSiv)))
        .push(btn("ChaCha20Poly1305", Message::SelectMethod(Algorithm::ChaCha20Poly1305)))
        .padding(Padding::from([40, 20]))
        .align_x(Alignment::Center)
        .spacing(30);

    container(column)
        .padding(Padding::from(20))
        .style(|_theme: &Theme| {apperance()})
}

fn passpharse_page(pass: Passpharse, is_checked: bool, warning: Option<String>) -> Container<'static, Message> { //passpharse page
    let pass_confirm = pass.confirm.clone();

    let column = Column::new()
        .push(text(warning.unwrap_or_else(|| "Please enter the passphrase:".to_string())).size(20))
        .push(
            input_field("Passphase...", &pass.passphase)
            .on_input(move |passpharse|{
                Message::PassphaseEnter(passpharse, pass.confirm.clone())
            })
            .secure(!is_checked)
        )
        .push(
            input_field("Confirm Passpharse...", &pass_confirm)
            .on_input(move |confirm|{
                Message::PassphaseEnter(pass.passphase.clone(), confirm)
            })
            .secure(!is_checked)
        )
        .push(checkbox("Show Password", is_checked).on_toggle(Message::ShowPass))
        .push(btn("Submit", Message::PasspharseSubmit))
        .padding(Padding::from([30, 20]))
        .align_x(Alignment::Center)
        .spacing(30);

    container(column)
        .padding(Padding::from(20))
        .style(|_theme: &Theme| {apperance()})
}

fn result(message: Option<String>) -> Container<'static, Message> { //show result (successful or not) of the encryption and decryption
    let column = Column::new()
        .push(text(message.unwrap()).size(20))
        .push(btn("Back", Message::Back))
        .padding(Padding::from([50, 20]))
        .align_x(Alignment::Center)
        .spacing(30);
    
    container(column)
        .width(Length::Fixed(500.0))
        .padding(Padding::from(20))
        .style(|_theme: &Theme| {apperance()})
}

fn read(content: String) -> Container<'static, Message> { //page for placing the content of the read file
    let column = Column::new()
        .push(scrollable(Column::new()
        .push(text(content).size(20))
        .width(900))
        .height(300))
        .push(btn("Back", Message::Back))
        .padding(Padding::from([50, 20]))
        .align_x(Alignment::Center)
        .spacing(30);
        
    container(column)
        .width(Length::Fixed(1000.0))
        .height(Length::Fixed(500.0))
        .padding(Padding::from(10))
        .align_x(Alignment::Center)
        .style(|_theme: &Theme| {apperance()})
}

fn apperance() -> container::Style {
    container::Style {
        text_color: Default::default(),
        border: Border::default().rounded(5),
        background: None,
        shadow: Shadow {
            color: Color::BLACK,
            offset:Vector::new(0.0, 2.0),
            blur_radius: 40.0,
        },
    }
}

fn btn(name: &str, event: Message) -> Button<Message> { //Creating the mainly used button
    Button::new(text(name).center().size(25))
    .on_press(event)
    .width(Length::Fixed(450.0))
    .height(Length::Fixed(70.0))
    .style(|theme: &Theme, status| {btn_style(theme, status, ButtonStyle::Standard)})
}

pub enum ButtonStyle{ //Types of button's style
    Standard,
    ThemeButton,
}

fn btn_style(theme: &Theme, status: Status, style: ButtonStyle) -> button::Style { //Custom Button's style
    match status {
        Status::Active => button::Style {
            background: Some(iced::Background::Color(match style {
                ButtonStyle::Standard => Color::BLACK,
                ButtonStyle::ThemeButton => Color::default(),
            })),
            border: match style {
                ButtonStyle::Standard => Border::default().rounded(20),
                ButtonStyle::ThemeButton => Border::default(),
            },
            text_color: Color::WHITE,
            shadow: match style {
                ButtonStyle::Standard => Shadow {
                    color: Color::BLACK,
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 20.0,
                },
                ButtonStyle::ThemeButton => Shadow::default(),
            }
        },
        Status::Hovered => button::Style {
            background: Some(iced::Background::Color(match style {
                ButtonStyle::Standard => Color::WHITE,
                ButtonStyle::ThemeButton => Color::default(),
            })),
            border: match style {
                ButtonStyle::Standard => Border::default().rounded(20),
                ButtonStyle::ThemeButton => Border::default(),
            },
            text_color: Color::BLACK,
            shadow: match style {
                ButtonStyle::Standard => Shadow {
                    color: Color::BLACK,
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 20.0,
                },
                ButtonStyle::ThemeButton => Shadow::default(),
            }
        },
        Status::Pressed => button::Style {
            background: Some(iced::Background::Color(match style {
                ButtonStyle::Standard => Color::default(),
                ButtonStyle::ThemeButton => Color::default(),
            })),
            border: match style {
                ButtonStyle::Standard => Border::default().rounded(20),
                ButtonStyle::ThemeButton => Border::default(),
            },
            text_color: Color::WHITE,
            shadow: match style {
                ButtonStyle::Standard => Shadow {
                    color: Color::BLACK,
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 20.0,
                },
                ButtonStyle::ThemeButton => Shadow::default(),
            }
        },
        _ => button::primary(theme, status),
    }
}

fn input_field(_placeholder: &str, _value: &str) -> TextInput<'static, Message> { //Create and receive input from the textbox
    TextInput::new(_placeholder, _value)
        .width(Length::Fixed(500.0))
        .padding(Padding::from(10))
        .line_height(text::LineHeight::Relative(1.75))
}

fn title(heading: &str) -> Text { //Page title custom
    Text::new(heading).size(100)
}

fn back_button(page: &Page) -> Button<'static, Message> { //Define which page it suppose to go back when hit the button
    Button::new(text("<-").align_x(Alignment::Start).size(50))
            .on_press(match page {
                Page::Menu => Message::Exit,
                Page::SelectFile(_command) => Message::ChangePage(Page::Menu),
                Page::SelectMethod => Message::ChangePage(Page::SelectFile(Command::Encrpyt)),
                Page::Passpharse(command) => match command {
                    Command::Encrpyt => Message::ChangePage(Page::SelectMethod),
                    _ => Message::ChangePage(Page::SelectFile(command.clone())),
                },
                _ => Message::ChangePage(Page::Menu)
            })
            .style(|theme: &Theme, status| {btn_style(theme, status, ButtonStyle::ThemeButton)})
}

impl FileEncryptor {
    pub fn new() -> Self {
        Self {
            page: Page::Menu,
            passphase: Passpharse {
                passphase: String::new(),
                confirm: String::new(),
            },
            command: None,
            selected_file: None,
            method: None,
            show_pass: false,
            message: None,
            content: String::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::PasspharseSubmit => { //This is when the user press the submit button on the passpharse page
                if self.passphase.passphase != self.passphase.confirm { //check if the passpharse and the confirm passpharse match
                    self.message = Some(String::from("Mismatch passphrases")) //if not send a message the user
                } else {
                    self.message = None;
                    if self.command != Some(Command::Read) { //I want the one with Encrypt and Decrypt command to go the result page
                        self.page = Page::Process(self.command.clone().unwrap());
                    }

                    match self.command { //perform the task according to the command
                        Some(Command::Encrpyt) => {
                            match backend::encrypt_file(self.selected_file.as_ref().unwrap(), self.selected_file.as_ref().unwrap(), &self.passphase.passphase, self.method.clone().unwrap()) {
                                Ok(_) => self.message = Some(String::from("Encrypted File Succesfully")),
                                Err(_) => self.message = Some(String::from("Failed to Encrypt")),
                            }
                        },
                        Some(Command::Decrypt) => {
                            match backend::create_decrypted_file(self.selected_file.as_ref().unwrap(), self.selected_file.as_ref().unwrap(), &self.passphase.passphase) {
                                Ok(_) => self.message = Some(String::from("Decrypted File Succesfully")),
                                Err(_) => self.message = Some(String::from("Incorrect Passpharse")),
                            }
                        },
                        Some(Command::Read) => {
                            match backend::read_file(self.selected_file.as_ref().unwrap(), &self.passphase.passphase) {
                                Ok(content) => {
                                    self.content = content;
                                    self.page = Page::Read;
                                },
                                Err(_) => self.message = Some(String::from("Unable to decrypt or incorrect passpharse"))
                            }
                        },
                        None => (),
                    }
                }
            },
            Message::PassphaseEnter(passpharse, confirm) => { //Storing the input from the passpharse textbox
                self.passphase.passphase = passpharse;
                self.passphase.confirm = confirm;
            },
            Message::ChangePage(page) => { //Logic for page change
                self.page = page;
                self.command = match &self.page {
                    Page::Menu => None,
                    Page::SelectFile(command) => Some(command.clone()),
                    Page::Passpharse(command) => Some(command.clone()),
                    Page::Process(command) => Some(command.clone()),
                    Page::Read => Some(Command::Read),
                    Page::SelectMethod => Some(Command::Encrpyt),
                }
            },
            Message::SelectFile => { //Selecting a file using rfd
                let file_path = if self.command == Some(Command::Read) { //Can only pick txt file when command is read
                    FileDialog::new().add_filter("text", &["txt"]).pick_file()
                } else {
                    FileDialog::new().pick_file()
                };
                self.selected_file = file_path;
            },
            Message::SelectMethod(method) => { //Storing which Algorithm the user pick
                self.method = Some(method);
                self.page = Page::Passpharse(Command::Encrpyt);
            },
            Message::ShowPass(is_checked) => { self.show_pass = is_checked }, //Show or Hide passpharse
            Message::Back => { //Logic for the back button
                self.page = Page::Menu;
                self.passphase.passphase = String::new();
                self.passphase.confirm = String::new();
                self.command = None;
                self.message = None;
                self.method = None;
                self.selected_file = None;
                self.show_pass = false;
            }
            Message::Exit => process::exit(0),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let content = match &self.page {
            Page::Menu => menu_page(),
            Page::SelectFile(command) => select_file(command.clone(), self.selected_file.clone()),
            Page::SelectMethod => select_method(),
            Page::Passpharse(_command) => passpharse_page(self.passphase.clone(), self.show_pass.clone(), self.message.clone()),
            Page::Process(_command) => result(self.message.clone()),
            Page::Read => read(self.content.clone()),
            _ => menu_page(),
        };

        let back = back_button(&self.page);
            

        let center = Column::new()
            .spacing(40)
            .width(Length::Fill)
            .push({
                if let Some(command) = &self.command {
                    match command {
                        Command::Read => title("Read File"),
                        Command::Encrpyt => title("Encrypt"),
                        Command::Decrypt => title("Decrypt"),
                    }
                } else {
                    title("FileEncryptor")
                }
            })
            .push(content)
            .align_x(Alignment::Center);

        let wrapper = match &self.page {
            Page::Process(_)
            | Page::Read => { Row::new()
                .width(Length::Fill)
                .push(center)
                .align_y(Alignment::Start)
                .padding(Padding::from(10))},
            _ => { Row::new()
                .width(Length::Fill)
                .push(back)
                .push(center)
                .push(Column::new().width(Length::Fixed(45.0)))
                .align_y(Alignment::Start)
                .padding(Padding::from(10))
            }
        };

        container(wrapper)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding::from(20))
            .center(Length::Fill)
            .into()
    }
}

impl Default for FileEncryptor {
    fn default() -> Self {
        Self::new()
    }
}