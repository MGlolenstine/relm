use crate::create_component;
use crate::Component;
use crate::UpdateNew;
use crate::{connect, widget::Widget, Relm, Update};
use gtk4::{prelude::*, Window};

pub enum AppMsg {
    Activate(Component<ApplicationWindow>),
    Run,
}

impl crate::DisplayVariant for AppMsg {
    #[allow(unused_qualifications)]
    fn display_variant(&self) -> &'static str {
        match *self {
            AppMsg::Activate { .. } => "Activate",
            AppMsg::Run { .. } => "Run",
        }
    }
}

impl crate::IntoOption<AppMsg> for AppMsg {
    fn into_option(self) -> Option<AppMsg> {
        Some(self)
    }
}

pub struct AppModel {
    app: gtk4::Application,
    win: Option<Component<ApplicationWindow>>,
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
                self.model.win.as_ref().unwrap().widget().show();
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
            let window = gtk4::ApplicationWindow::new(&app);
            let component = create_component::<ApplicationWindow>(window);
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

pub struct AppWindowModel {
    win: gtk4::ApplicationWindow,
}

pub struct ApplicationWindow {
    model: AppWindowModel,
}

impl Widget for ApplicationWindow {
    type Root = gtk4::ApplicationWindow;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.model.win.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        ApplicationWindow { model }
    }
}

impl Update for ApplicationWindow {
    type Model = AppWindowModel;
    type ModelParam = gtk4::ApplicationWindow;
    type Msg = AppMsg;

    fn model(_: &Relm<Self>, win: gtk4::ApplicationWindow) -> Self::Model {
        Self::Model { win }
    }

    fn update(&mut self, event: AppMsg) {
        // match event {
        //     AppMsg::Activate(component) => {
        //         println!("Activate");
        //         //self.model.win = Some(create_component::<Win>(self.model.app.clone()));
        //         self.model.win = Some(component);
        //         println!("After Activate");
        //     }
        //     AppMsg::Run => {
        //         println!("Run");
        //         //self.model.app.run();
        //     }
        // }
    }
}
