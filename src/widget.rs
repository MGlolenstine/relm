/*
 * Copyright (c) 2017-2018 Boucher, Antoni <bouanto@zoho.com>
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

use super::{run, Relm};
use crate::state::Update;

/// Trait to implement to manage widget's events.
pub trait Widget
where
    Self: Update,
    Self::Root: Clone + IsA<Object> + IsA<gtk4::Widget>,
{
    /// The type of the root widget.
    type Root;

    /// Update the view after it is initially created.
    /// This method is only useful when using the `#[widget]` attribute, because when not using it,
    /// you can use the [`view()`](trait.Widget.html#tymethod.view) method instead.
    fn init_view(&mut self) {}

    /// Method called when the widget is added to its parent.
    /// This is currently only used to set the child properties of a widget as relm widget could
    /// have child properties and we don't know its parent when it is defined. Thus, we call
    /// on_add() when it is added to its parent to set the child properties.
    fn on_add<W: IsA<gtk4::Widget> + IsA<Object>>(&self, _parent: W) {}

    /// Get the parent ID.
    /// This is useful for custom Container implementation: when you implement the
    /// [`Container::add_widget()`](trait.Container.html#tymethod.add_widget), you might want to
    /// insert widgets elsewhere depending of this id.
    fn parent_id() -> Option<&'static str> {
        None
    }

    // TODO: ajouter une méthode param() pour déterminer des paramètres qui seront pris en compte à
    // l’ajout du widget.

    /// Get the root widget of the view.
    fn root(&self) -> Self::Root;

    /// Create the window from this widget and start the main loop.
    fn run(app: gtk4::Application, model_param: Self::ModelParam) -> Result<(), glib::BoolError>
    where
        Self: 'static,
        <Self as crate::state::Update>::ModelParam: Clone,
    {
        run::<Self>(app, model_param)
    }

    /// Create the initial view.
    fn view(relm: &Relm<Self>, model: Self::Model) -> Self;
}

/// Trait implemented by the generator to ease the creation of tests of relm widgets using the
/// view! macro.
pub trait WidgetTest: Widget {
    /// Represents the structure holding all the `StreamHandle`s. Useful for tests.
    type Streams;
    /// Represents the structure holding all the widgets. Useful for tests.
    type Widgets;

    /// Get the structure containing all the `StreamHandle`s. Useful for tests.
    fn get_streams(&self) -> Self::Streams;

    /// Get the structure containing all the widgets. Useful for tests.
    fn get_widgets(&self) -> Self::Widgets;
}
