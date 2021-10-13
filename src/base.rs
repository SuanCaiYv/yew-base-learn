use yew::{Component, ComponentLink, Html, html, ShouldRender, Properties};

pub struct MyProps {
}

impl Clone for MyProps {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Properties for MyProps {
    type Builder = ();

    fn builder() -> Self::Builder {
        todo!()
    }
}

pub struct MyComponent {
    // 接收状态
    pros: MyProps,
    // 注册回调或发消息给这个组件之类的
    link: ComponentLink<Self>,
}

pub enum MyMsg {
    Default
}

impl From<String> for MyMsg {
    fn from(_: String) -> Self {
        MyMsg::Default
    }
}

impl Component for MyComponent {
    // 用于在事件发生后，向组件传递消息
    type Message = MyMsg;
    // 用于父组件给子组件传递属性值
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        println!("run0");
        // 向组件发送消息，并会触发update来处理消息
        // link.send_message()
        // let res = link.callback("");
        // 通过emit(value T)函数发送消息给回调，回调会调用传入的闭包处理
        // res.emit()
        let callback = link.callback(|input: i32| -> String {
            let mut ans = String::new();
            ans.push_str("Num: ");
            ans.push_str(input.to_string().as_str());
            return ans
        });
        callback.emit(12);
        Self {
            pros: MyProps{},
            link,
        }
    }

    // 由消息触发，决定是否更新组件，并重新渲染
    // 对于每个消息都会被调用。这使得组件可以根据消息的内容来更新自身，并决定是否需要重新渲染自己
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        println!("run1");
        true
    }

    // 由父节点触发，根据收到的新的属性，决定是否重新渲染
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        println!("run2");
        true
    }

    // 声明组建的布局
    fn view(&self) -> Html {
        html! {
            <>
                <div alien="center">
                    <p align="center">{"Hello, Yew"}</p>
                </div>
            </>
        }
    }

    // 渲染函数，调用完view并且 Yew 将结果渲染到 DOM ，而浏览器还未刷新页面的时候调用这个函数
    fn rendered(&mut self, _first_render: bool) {
        println!("run3");
        todo!()
    }

    fn destroy(&mut self) {
        todo!()
    }
}