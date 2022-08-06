pub mod wasm;

use crate::dom;
use maud::html;
use maud::Markup;

pub struct PageMarkup {
    pub head: Markup,
    pub body: Markup,
}

impl PageMarkup {
    pub fn to_markup(&self) -> Markup {
        html! {
            (maud::DOCTYPE)
            html {
                head {
                    meta charset="utf-8";
                    (self.head)
                }
                body {
                    (self.body)
                }
            }
        }
    }
}

pub trait Page<Model, Msg> {
    fn id(&self) -> dom::DomId;
    fn initial_model(&self) -> Model;
    fn effects(&self, model: &Model) -> dom::Effects<Msg>;
    fn update(&self, msg: &Msg, model: &mut Model) -> Result<(), String>;
    fn view(&self, model: &Model) -> PageMarkup;
}
