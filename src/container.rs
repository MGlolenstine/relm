/*
 * Copyright (c) 2017 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

use glib::{IsA, Object};
use gtk4::prelude::WidgetExt;

use super::{create_widget, init_component, Component, DisplayVariant};
use crate::state::EventStream;
use crate::widget::Widget;

/// Struct for relm containers to add GTK+ and relm `Widget`s.
pub struct ContainerComponent<WIDGET: Container + Widget> {
    component: Component<WIDGET>,
    /// The default container of this component.
    pub container: WIDGET::Container,
    /// Additional containers used for multi-containers. This can be () if not needed.
    pub containers: WIDGET::Containers,
}

impl<WIDGET: Container + Widget> Clone for ContainerComponent<WIDGET> {
    fn clone(&self) -> Self {
        Self {
            component: self.component.clone(),
            container: self.container.clone(),
            containers: self.containers.clone(),
        }
    }
}

impl<WIDGET: Container + Widget> ContainerComponent<WIDGET> {
    #[doc(hidden)]
    pub fn new(
        component: Component<WIDGET>,
        container: WIDGET::Container,
        containers: WIDGET::Containers,
    ) -> Self {
        ContainerComponent {
            component,
            container,
            containers,
        }
    }

    /// Add a GTK+ widget to a relm container.
    pub fn add<CHILDWIDGET: IsA<gtk4::Widget>>(&self, _widget: &CHILDWIDGET) {
        unimplemented!("Add #1");
        // self.container.add(widget);
    }

    /// Add a relm widget to a relm container.
    pub fn add_widget<CHILDWIDGET>(
        &self,
        _model_param: CHILDWIDGET::ModelParam,
    ) -> Component<CHILDWIDGET>
    where
        CHILDWIDGET: Widget + 'static,
        WIDGET::Container: ContainerExt + IsA<gtk4::Widget> + IsA<Object>,
    {
        // let (component, widget, child_relm) = create_widget::<CHILDWIDGET>(model_param);
        // let container = WIDGET::add_widget(self, &component);
        // widget.on_add(container);
        // init_component::<CHILDWIDGET>(component.stream(), widget, &child_relm);
        // component
        unimplemented!("Add #2");
    }

    /// Emit a message of the widget stream.
    pub fn emit(&self, msg: WIDGET::Msg) {
        self.stream().emit(msg);
    }

    /// Get the event stream of the component.
    /// This is used internally by the library.
    pub fn stream(&self) -> &EventStream<WIDGET::Msg> {
        self.component.stream()
    }

    // TODO: add delete methods?

    /// Get the widget of the component.
    pub fn widget(&self) -> &WIDGET::Root {
        self.component.widget()
    }
}

/// Trait to implement relm container widget.
pub trait Container: Widget {
    /// The type of the containing widget, i.e. where the child widgets will be added.
    type Container: Clone + IsA<Object> + IsA<gtk4::Widget>;
    /// Type to contain the additional container widgets.
    // TODO: put that in yet another trait?
    type Containers: Clone;

    /// Add a relm widget to this container.
    /// Return the widget that will be send to Widget::on_add().
    //TODO: Cast it into some kind of a container, probably creating a Container trait for grouping them together.
    // fn add_widget<WIDGET: Widget>(
    //     container: &ContainerComponent<Self>,
    //     component: &Component<WIDGET>,
    // ) -> Self::Container {
    //     container.container.add(component.widget().clone());
    //     container.container.clone().upcast()
    // }

    /// Get the containing widget, i.e. the widget where the children will be added.
    fn container(&self) -> &Self::Container;

    /// Get additional container widgets.
    /// This is useful to create a multi-container.
    fn other_containers(&self) -> Self::Containers;
}

/// Extension trait for GTK+ containers to add and remove relm `Widget`s.
pub trait ContainerWidget {
    /// Add a relm `Container` to the current GTK+ container.
    ///
    /// # Note
    ///
    /// The returned `ContainerComponent` must be stored in a `Widget`. If it is not stored, a
    /// communication receiver will be droped which will cause events to be ignored for this
    /// widget.
    fn add_container<CHILDWIDGET>(
        &self,
        model_param: CHILDWIDGET::ModelParam,
    ) -> ContainerComponent<CHILDWIDGET>
    where
        CHILDWIDGET: Container + Widget + 'static,
        CHILDWIDGET::Msg: DisplayVariant + 'static,
        CHILDWIDGET::Root: IsA<gtk4::Widget> + IsA<Object> + WidgetExt;

    /// Add a relm `Widget` to the current GTK+ container.
    ///
    /// # Note
    ///
    /// The returned `Component` must be stored in a `Widget`. If it is not stored, a communication
    /// receiver will be droped which will cause events to be ignored for this widget.
    fn add_widget<CHILDWIDGET>(
        &self,
        model_param: CHILDWIDGET::ModelParam,
    ) -> Component<CHILDWIDGET>
    where
        CHILDWIDGET: Widget + 'static,
        CHILDWIDGET::Msg: DisplayVariant + 'static,
        CHILDWIDGET::Root: IsA<gtk4::Widget> + IsA<Object> + WidgetExt;

    /// Remove a relm `Widget` from the current GTK+ container.
    fn remove_widget<CHILDWIDGET>(&self, component: Component<CHILDWIDGET>)
    where
        CHILDWIDGET: Widget,
        CHILDWIDGET::Root: IsA<gtk4::Widget>;
}

impl<W: Clone + ContainerExt + IsA<gtk4::Widget> + IsA<Object>> ContainerWidget for W {
    fn add_container<CHILDWIDGET>(
        &self,
        model_param: CHILDWIDGET::ModelParam,
    ) -> ContainerComponent<CHILDWIDGET>
    where
        CHILDWIDGET: Container + Widget + 'static,
        CHILDWIDGET::Msg: DisplayVariant + 'static,
        CHILDWIDGET::Root: IsA<gtk4::Widget> + IsA<Object> + WidgetExt,
    {
        let (component, widget, child_relm) = create_widget::<CHILDWIDGET>(model_param);
        let container = widget.container().clone();
        let containers = widget.other_containers();
        let root = widget.root();
        self.add(&root);
        widget.on_add(self.clone());
        init_component::<CHILDWIDGET>(component.stream(), widget, &child_relm);
        ContainerComponent::new(component, container, containers)
    }

    fn add_widget<CHILDWIDGET>(
        &self,
        model_param: CHILDWIDGET::ModelParam,
    ) -> Component<CHILDWIDGET>
    where
        CHILDWIDGET: Widget + 'static,
        CHILDWIDGET::Msg: DisplayVariant + 'static,
        CHILDWIDGET::Root: IsA<gtk4::Widget> + IsA<Object> + WidgetExt,
    {
        let (component, widget, child_relm) = create_widget::<CHILDWIDGET>(model_param);
        self.add(component.widget());
        widget.on_add(self.clone());
        init_component::<CHILDWIDGET>(component.stream(), widget, &child_relm);
        component
    }

    fn remove_widget<WIDGET>(&self, component: Component<WIDGET>)
    where
        WIDGET: Widget,
        WIDGET::Root: IsA<gtk4::Widget>,
    {
        self.remove(component.widget());
    }
}

pub trait ContainerExt {
    fn remove<ROOT>(&self, widget: &ROOT)
    where
        ROOT: Clone + IsA<Object> + IsA<gtk4::Widget>;
    fn add<ROOT>(&self, widget: &ROOT)
    where
        ROOT: Clone + IsA<Object> + IsA<gtk4::Widget>;
}
