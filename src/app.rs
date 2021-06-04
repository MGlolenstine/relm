use crate::create_component;
use crate::Component;
use crate::UpdateNew;
use crate::{connect, widget::Widget, Relm, Update};
use gtk4::prelude::*;
use relm_derive::Msg;

#[derive(Msg)]
enum AppMsg {
    Activate(Component<dyn Widget>),
    Run,
}

struct AppModel {
    app: gtk4::Application,
    win: Option<Component<dyn Widget>>,
}

pub struct App {
    model: AppModel,
}

impl Update for App {
    type Model = AppModel;
    type ModelParam = gtk4::Application;
    type Msg = AppMsg;

    fn model(_: &Relm<Self>, app: gtk4::Application) -> AppModel {
        AppModel { app, win: None }
    }

    fn update(&mut self, event: AppMsg) {
        match event {
            AppMsg::Activate(component) => {
                println!("Activate");
                //self.model.win = Some(create_component::<Win>(self.model.app.clone()));
                self.model.win = Some(component);
                println!("After Activate");
            }
            AppMsg::Run => {
                println!("Run");
                //self.model.app.run();
            }
        }
    }

    fn subscriptions(&mut self, relm: &Relm<Self>) {
        let app = self.model.app.clone();
        connect!(self.model.app, connect_activate(_), relm, {
            let component = create_component::<dyn Widget>(app.clone());
            AppMsg::Activate(component)
        });
        //relm.stream().emit(AppMsg::Run);
    }
}

impl UpdateNew for App {
    fn new(_relm: &Relm<Self>, model: Self::Model) -> Self {
        App { model }
    }
}
