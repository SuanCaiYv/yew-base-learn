use yew::{html, Component, ComponentLink, InputData};

struct Text {
    link: ComponentLink<Self>,
    text: String,
}

enum Msg {
    Input(String),
}

impl Component for Text {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            link,
            text: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Input(val) => {
                self.text = val;
            }
            _ => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        html! {
            <>
                <input placeholder="Input a str" type="text" oninput=self.link.callback(|txt: InputData| -> Msg {
                    Msg::Input(txt.value)
                })/>
                <br/>
                {{"Output: "}}
                <b>{self.text.clone()}</b>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Text>();
}
