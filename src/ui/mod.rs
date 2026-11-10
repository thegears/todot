use crate::ui::modals::CurrentModal;

pub mod main_screen;
pub mod modals;

pub enum Action {
    None,
    Close,
    Open(CurrentModal),
}
