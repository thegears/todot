use crate::ui::modals::Modal;

pub mod main_screen;
pub mod modals;

pub enum Action {
    None,
    Close,
    Open(Box<dyn Modal>),
}
