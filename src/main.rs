use iced;
mod FileEncryptor;
use FileEncryptor::GUI;

fn main() -> iced::Result {
    iced::run("FileEncryptor", GUI::FileEncryptor::update, GUI::FileEncryptor::view)
}