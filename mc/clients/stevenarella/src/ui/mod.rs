// Copyright 2016 Matthew Collins
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod button;
mod container;
mod formatting;
mod image;
mod input;
mod layout;
mod text;
mod textbox;

pub mod logo;

use crate::format;
use crate::render;
#[cfg(not(target_arch = "wasm32"))]
use clipboard::{ClipboardContext, ClipboardProvider};
use std::cell::{RefCell, RefMut};
use std::rc::{Rc, Weak};
use winit::event::VirtualKeyCode;

pub use self::layout::{HAttach, Mode, VAttach};
use self::layout::{Region, WidgetGeometry, SCREEN};

const INITIAL_RENDER_VERSION: usize = 0xFFFF;

macro_rules! define_elements {
    (
        $($name:ident,)*
    ) => (
        #[doc(hidden)]
        pub enum Element {
            $($name(Rc<RefCell<$name>>),)*
        }

        impl Element {
            fn get_draw_index(&self) -> isize {
                match *self {
                    $(
                        Element::$name(ref inner) => inner.borrow().draw_index,
                    )*
                }
            }

            fn is_unused(&self) -> bool {
                match *self {
                    $(
                        Element::$name(ref inner) => Rc::strong_count(inner) == 1,
                    )*
                }
            }

            fn tick(&self, renderer: &mut render::Renderer) {
                match *self {
                    $(
                        Element::$name(ref inner) => {
                            let mut el = inner.borrow_mut();
                            el.tick(renderer);
                        },
                    )*
                }
            }

            fn draw(&self, renderer: &mut render::Renderer, r: &Region, sw: f64, sh: f64, width: f64, height: f64, delta: f64) -> RefMut<[u8]> {
                match *self {
                    $(
                        Element::$name(ref inner) => {
                            let el = inner.borrow_mut();
                            RefMut::map(el, |el| el.draw(renderer, r, sw, sh, width, height, delta))
                        },
                    )*
                }
            }

            fn check_rebuild(&self) -> bool {
                match *self {
                    $(
                        Element::$name(ref inner) => {
                            let el = inner.borrow();
                            el.check_rebuild()
                        },
                    )*
                }
            }

            fn force_rebuild(&self) {
                match *self {
                    $(
                        Element::$name(ref inner) => {
                            let mut el = inner.borrow_mut();
                            el.needs_rebuild = true;
                        },
                    )*
                }
            }

            fn get_size(&self) -> (f64, f64) {
                match *self {
                    $(
                        Element::$name(ref inner) => {
                            let el = inner.borrow();
                            el.get_size()
                        },
                    )*
                }
            }

            fn get_position(&self) -> (f64, f64) {
                match *self {
                    $(
                        Element::$name(ref inner) => {
                            let el = inner.borrow();
                            (el.x, el.y)
                        },
                    )*
                }
            }
            fn get_attachment(&self) -> (VAttach, HAttach) {
                match *self {
                    $(
                        Element::$name(ref inner) => {
                            let el = inner.borrow();
                            (el.v_attach, el.h_attach)
                        },
                    )*
                }
            }

            fn hover_at(&self, r: &Region, game: &mut crate::Game, mx: f64, my: f64, sw: f64, sh: f64) -> bool {
                match *self {
                    $(
                        Element::$name(ref inner) => {
                            let mut el = inner.borrow_mut();
                            el.hover_at(r, game, mx, my, sw, sh)
                        },
                    )*
                }
            }

            fn click_at(&self, r: &Region, game: &mut crate::Game, mx: f64, my: f64, sw: f64, sh: f64) -> bool {
                match *self {
                    $(
                        Element::$name(ref inner) => {
                            let mut el = inner.borrow_mut();
                            el.click_at(r, game, mx, my, sw, sh)
                        },
                    )*
                }
            }

            fn key_press(&self, game: &mut crate::Game, key: VirtualKeyCode, down: bool, ctrl_pressed: bool) {
                match *self {
                    $(
                        Element::$name(ref inner) => {
                            let mut el = inner.borrow_mut();
                            el.key_press(game, key, down, ctrl_pressed);
                        },
                    )*
                }
            }
            fn key_type(&self, game: &mut crate::Game, c: char) {
                match *self {
                    $(
                        Element::$name(ref inner) => {
                            let mut el = inner.borrow_mut();
                            el.key_type(game, c);
                        },
                    )*
                }
            }

            fn is_focused(&self) -> bool {
                match *self {
                    $(
                        Element::$name(ref inner) => {
                            let el = inner.borrow();
                            el.focused
                        },
                    )*
                }
            }

            fn set_focused(&self, val: bool) {
                match *self {
                    $(
                        Element::$name(ref inner) => {
                            let mut el = inner.borrow_mut();
                            el.focused = val;
                        },
                    )*
                }
            }
        }

        #[doc(hidden)]
        enum WeakElement {
            $($name(Weak<RefCell<$name>>),)*
        }

        impl WeakElement {
            fn upgrade(&self) -> Option<Element> {
                match *self {
                    $(
                        WeakElement::$name(ref inner) => {
                            inner.upgrade().map(Element::$name)
                        },
                    )*
                }
            }
        }
    )
}

define_elements! {
    Image,
    Batch,
    Text,
    Formatted,
    Button,
    TextBox,
}
pub trait ElementHolder {
    fn add(&mut self, el: Element, auto_free: bool);
}

pub struct Container {
    elements: Vec<Element>,
    focusable_elements: Vec<WeakElement>,

    pub mode: Mode,
    last_mode: Mode,
    version: usize,

    last_sw: f64,
    last_sh: f64,
    last_width: f64,
    last_height: f64,
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl Container {
    pub fn new() -> Container {
        Container {
            elements: Vec::new(),
            focusable_elements: Vec::new(),

            mode: Mode::Scaled,
            last_mode: Mode::Scaled,
            version: INITIAL_RENDER_VERSION,

            last_sw: 0.0,
            last_sh: 0.0,
            last_width: 0.0,
            last_height: 0.0,
        }
    }

    pub fn tick(&mut self, renderer: &mut render::Renderer, delta: f64, width: f64, height: f64) {
        let (sw, sh) = layout::scale_for_mode(self.mode, width, height);

        if self.last_sw != sw
            || self.last_sh != sh
            || self.last_width != width
            || self.last_height != height
            || self.version != renderer.ui.version
            || self.last_mode != self.mode
        {
            self.last_sw = sw;
            self.last_sh = sh;
            self.last_width = width;
            self.last_height = height;
            self.last_mode = self.mode;
            for e in &self.elements {
                e.force_rebuild();
            }
            self.version = renderer.ui.version;
        }

        // Drop elements with no refs
        self.elements.retain(|v| !v.is_unused());
        // Drop focusable elements that no longer exist
        self.focusable_elements.retain(|v| v.upgrade().is_some());

        // If we don't have an element focused, focus one
        let any_focused = self
            .focusable_elements
            .iter()
            .flat_map(|v| v.upgrade())
            .any(|v| v.is_focused());
        if input::should_auto_focus(self.focusable_elements.len(), any_focused) {
            self.cycle_focus()
        }

        for e in &self.elements {
            e.tick(renderer);
        }
        for e in &self.elements {
            let r = Self::compute_draw_region(e, sw, sh, &SCREEN);
            if r.intersects(&SCREEN) {
                let data = e.draw(renderer, &r, sw, sh, width, height, delta);
                renderer.ui.add_bytes(&data);
            }
        }
    }

    pub fn hover_at(&mut self, game: &mut crate::Game, x: f64, y: f64, width: f64, height: f64) {
        let (sw, sh) = layout::scale_for_mode(self.mode, width, height);
        let (mx, my) = layout::mouse_to_scaled(x, y, width, height);

        for e in &self.elements {
            let r = Self::compute_draw_region(e, sw, sh, &SCREEN);
            e.hover_at(&r, game, mx, my, sw, sh);
        }
    }

    pub fn click_at(&mut self, game: &mut crate::Game, x: f64, y: f64, width: f64, height: f64) {
        let (sw, sh) = layout::scale_for_mode(self.mode, width, height);
        let (mx, my) = layout::mouse_to_scaled(x, y, width, height);

        let mut clicked_element: Option<&Element> = None;
        for e in &self.elements {
            let r = Self::compute_draw_region(e, sw, sh, &SCREEN);
            if r.contains(mx, my) {
                e.click_at(&r, game, mx, my, sw, sh);
                clicked_element = Some(e);
            }
        }

        if let Some(e) = clicked_element {
            if let Element::TextBox(_) = e {
                self.clear_focus();
                e.set_focused(true);
            }
        }
    }

    fn add_focusable(&mut self, el: WeakElement) {
        self.focusable_elements.push(el);
    }

    fn clear_focus(&self) {
        for e in &self.elements {
            e.set_focused(false);
        }
    }

    pub fn cycle_focus(&mut self) {
        let focusables = self
            .focusable_elements
            .iter()
            .flat_map(|v| v.upgrade())
            .collect::<Vec<_>>();

        // Find the last focused element if there is one
        let last_focus = focusables.iter().position(|v| v.is_focused());
        let next_focus = match input::next_focus_index(last_focus, focusables.len()) {
            Some(index) => index,
            None => return,
        };

        // Clear the last focus
        if let Some(focus) = last_focus {
            focusables[focus].set_focused(false);
        }

        focusables[next_focus].set_focused(true);
    }

    pub fn key_press(
        &mut self,
        game: &mut crate::Game,
        key: VirtualKeyCode,
        down: bool,
        ctrl_pressed: bool,
    ) {
        if key == VirtualKeyCode::Tab {
            if !down {
                self.cycle_focus();
            }
            return;
        }
        for el in self.focusable_elements.iter().flat_map(|v| v.upgrade()) {
            if el.is_focused() {
                el.key_press(game, key, down, ctrl_pressed);
            }
        }
    }

    pub fn key_type(&mut self, game: &mut crate::Game, c: char) {
        if !input::accepts_typed_char(c) {
            return;
        }
        for el in self.focusable_elements.iter().flat_map(|v| v.upgrade()) {
            if el.is_focused() {
                el.key_type(game, c);
            }
        }
    }

    fn compute_draw_region(el: &Element, sw: f64, sh: f64, super_region: &Region) -> Region {
        let (width, height) = el.get_size();
        let (x, y) = el.get_position();
        let (v_attach, h_attach) = el.get_attachment();
        layout::compute_draw_region(
            WidgetGeometry {
                x,
                y,
                width,
                height,
                v_attach,
                h_attach,
            },
            (sw, sh),
            super_region,
        )
    }
}

impl ElementHolder for Container {
    fn add(&mut self, el: Element, auto_free: bool) {
        if !auto_free {
            panic!("Auto free elements are not allowed on root");
        }
        self.elements.push(el);
        container::sort_by_draw_index(&mut self.elements, |v| v.get_draw_index());
    }
}

trait UIElement {
    fn draw(
        &mut self,
        renderer: &mut render::Renderer,
        r: &Region,
        sw: f64,
        sh: f64,
        width: f64,
        height: f64,
        delta: f64,
    ) -> &mut [u8];
    fn get_size(&self) -> (f64, f64);
    fn is_dirty(&self) -> bool;
    fn post_init(_: Rc<RefCell<Self>>) {}
    fn key_press(
        &mut self,
        _game: &mut crate::Game,
        _key: VirtualKeyCode,
        _down: bool,
        _ctrl_pressed: bool,
    ) {
    }
    fn key_type(&mut self, _game: &mut crate::Game, _c: char) {}
    fn tick(&mut self, renderer: &mut render::Renderer);
}

macro_rules! element {
    (
        ref $nameref:ident
        pub struct $name:ident {
            $(pub $pfield_name:ident : $pfield_type:ty,)*
            $(priv $field_name:ident : $field_type:ty,)*
        }

        builder $builder:ident {
            $(hardcode $hname:ident = $hval:expr,)*
            $(simple $sname:ident : $sty:ty,)*
            $(optional $oname:ident : $oty:ty = $oval:expr,)*
            $(noset $nname:ident : $nty:ty = |$bref:ident| $nval:expr,)*
        }
    ) => (
        pub struct $name {
            $(pub $pfield_name : $pfield_type,)*
            $($field_name : $field_type,)*
            // Base fields
            draw_index: isize,
            elements: Vec<(bool, Element)>,
            pub x: f64,
            pub y: f64,
            pub v_attach: VAttach,
            pub h_attach: HAttach,
            data: Vec<u8>,
            needs_rebuild: bool,

            hover_funcs: Vec<Box<dyn Fn(&mut $name, bool, &mut crate::Game) -> bool>>,
            hover_state: bool,
            click_funcs: Vec<Box<dyn Fn(&mut $name, &mut crate::Game) -> bool>>,

            focused: bool,

            // Change checking
            last_x: f64,
            last_y: f64,
            last_v_attach: VAttach,
            last_h_attach: HAttach,
            last_width: f64,
            last_height: f64,
        }

        pub type $nameref = Rc<RefCell<$name>>;

        impl ElementHolder for $name {
            fn add(&mut self, el: Element, auto_free: bool) {
                self.elements.push((auto_free, el));
                container::sort_by_draw_index(&mut self.elements, |v| v.1.get_draw_index());
            }
        }

        impl $name {
            fn check_rebuild(&self) -> bool {
                if self.needs_rebuild {
                    return true;
                }
                // Check for changes that would cause child
                // elements to need an update
                let (w, h) = self.get_size();
                if self.last_x != self.x || self.last_y != self.y
                        || self.last_width != w || self.last_height != h
                        || self.last_v_attach != self.v_attach || self.last_h_attach != self.h_attach {
                    return true;
                }
                if self.is_dirty() {
                    return true;
                }
                for e in &self.elements {
                    if e.1.check_rebuild() {
                        return true;
                    }
                }
                false
            }

            fn super_draw(&mut self, renderer: &mut render::Renderer, super_region: &Region, sw: f64, sh: f64, width: f64, height: f64, delta: f64) {
                if !self.needs_rebuild {
                    let (w, h) = self.get_size();
                    self.needs_rebuild = self.last_x != self.x || self.last_y != self.y
                                || self.last_width != w || self.last_height != h
                                || self.last_v_attach != self.v_attach || self.last_h_attach != self.h_attach;
                }
                self.elements.retain(|v| !v.0 || !v.1.is_unused());
                for &(_, ref e) in &self.elements {
                    if self.needs_rebuild {
                        e.force_rebuild();
                    }
                    let r = Container::compute_draw_region(e, sw, sh, &super_region);
                    let data = e.draw(renderer, &r, sw, sh, width, height, delta);
                    self.data.extend_from_slice(&data);
                }
                self.needs_rebuild = false;
                self.last_x = self.x;
                self.last_y = self.y;
                let (w, h) = self.get_size();
                self.last_width = w;
                self.last_height = h;
                self.last_v_attach = self.v_attach;
                self.last_h_attach = self.h_attach;
            }

            fn super_tick(&mut self, renderer: &mut render::Renderer) {
                for &(_, ref e) in &self.elements {
                    e.tick(renderer);
                }
            }

            fn hover_at(&mut self, super_region: &Region, game: &mut crate::Game, mx: f64, my: f64, sw: f64, sh: f64) -> bool {
                use std::mem;
                let mut handle_self = true;
                for e in &self.elements {
                    let r = Container::compute_draw_region(&e.1, sw, sh, &super_region);
                    if e.1.hover_at(&r, game, mx, my, sw, sh) {
                        handle_self = false;
                    }
                }
                if handle_self {
                    let state = super_region.contains(mx, my);
                    if state != self.hover_state {
                        self.hover_state = state;
                        let len = self.hover_funcs.len();
                        let mut temp = mem::replace(&mut self.hover_funcs, Vec::with_capacity(len));
                        let mut block_prop = false;
                        for func in &temp {
                            block_prop |= (func)(self, state, game);
                        }
                        self.hover_funcs.append(&mut temp);
                        block_prop
                    } else {
                        false
                    }
                } else {
                    true // Carry up
                }
            }

            pub fn add_hover_func<F: Fn(&mut $name, bool, &mut crate::Game) -> bool + 'static>(&mut self, func: F) {
                self.hover_funcs.push(Box::new(func));
            }

            fn click_at(&mut self, super_region: &Region, game: &mut crate::Game, mx: f64, my: f64, sw: f64, sh: f64) -> bool {
                use std::mem;
                let mut handle_self = true;
                for e in &self.elements {
                    let r = Container::compute_draw_region(&e.1, sw, sh, &super_region);
                    if r.contains(mx, my) && e.1.click_at(&r, game, mx, my, sw, sh) {
                        handle_self = false;
                    }
                }
                if handle_self {
                    let len = self.click_funcs.len();
                    let mut temp = mem::replace(&mut self.click_funcs, Vec::with_capacity(len));
                    let mut block_prop = false;
                    for func in &temp {
                        block_prop |= (func)(self, game);
                    }
                    self.click_funcs.append(&mut temp);
                    block_prop
                } else {
                    true // Carry up
                }
            }

            pub fn add_click_func<F: Fn(&mut $name, &mut crate::Game) -> bool + 'static>(&mut self, func: F) {
                self.click_funcs.push(Box::new(func));
            }

            pub fn make_focusable(this: &$nameref, container: &mut Container) {
                container.add_focusable(WeakElement::$name(Rc::downgrade(&this)));
            }
        }

        #[derive(Default)]
        pub struct $builder {
            $(
                $sname: Option<$sty>,
            )*
            $(
                $oname: Option<$oty>,
            )*
            $(
                $nname: Option<$nty>,
            )*
            // Base fields
            draw_index: isize,
            x: Option<f64>,
            y: Option<f64>,
            v_attach: Option<VAttach>,
            h_attach: Option<HAttach>,
        }

        impl $builder {
            $(
                pub fn $sname<T: Into<$sty>>(mut self, val: T) -> Self {
                    self.$sname = Some(val.into());
                    self
                }
            )*
            $(
                pub fn $oname<T: Into<$oty>>(mut self, val: T) -> Self {
                    self.$oname = Some(val.into());
                    self
                }
            )*

            // Base fields
            pub fn draw_index(mut self, draw_index: isize) -> Self {
                self.draw_index = draw_index;
                self
            }

            pub fn position(mut self, x: f64, y: f64) -> Self {
                self.x = Some(x);
                self.y = Some(y);
                self
            }

            pub fn alignment(mut self, v_attach: VAttach, h_attach: HAttach) -> Self {
                self.v_attach = Some(v_attach);
                self.h_attach = Some(h_attach);
                self
            }

            pub fn new() -> Self {
                $builder {
                    $(
                        $sname: None,
                    )*
                    $(
                        $oname: None,
                    )*
                    $(
                        $nname: None,
                    )*
                    draw_index: 0,
                    x: None,
                    y: None,
                    v_attach: None,
                    h_attach: None,
                }
            }

            pub fn create<H: ElementHolder>(self, ui: &mut H) -> $nameref {
                self.create_internal(ui, true)
            }

            pub fn attach<H: ElementHolder>(self, ui: &mut H) -> $nameref {
                self.create_internal(ui, false)
            }

            fn create_internal<H: ElementHolder>(self, ui: &mut H, auto_free: bool) -> $nameref {
                $(
                    let $nname = {let $bref = &self; $nval};
                )*
                let v = Rc::new(RefCell::new($name {
                    $(
                        $hname: $hval,
                    )*
                    $(
                        $sname: self.$sname.expect(concat!("Missing required field ", stringify!($sname))),
                    )*
                    $(
                        $oname: self.$oname.unwrap_or($oval),
                    )*
                    $(
                        $nname,
                    )*
                    // Base fields
                    draw_index: self.draw_index,
                    elements: Vec::new(),
                    x: self.x.unwrap_or(0.0),
                    y: self.y.unwrap_or(0.0),
                    v_attach: self.v_attach.unwrap_or(VAttach::Top),
                    h_attach: self.h_attach.unwrap_or(HAttach::Left),
                    last_x: self.x.unwrap_or(0.0),
                    last_y: self.y.unwrap_or(0.0),
                    last_v_attach: self.v_attach.unwrap_or(VAttach::Top),
                    last_h_attach: self.h_attach.unwrap_or(HAttach::Left),
                    last_width: 0.0,
                    last_height: 0.0,
                    data: vec![],
                    needs_rebuild: true,

                    hover_funcs: vec![],
                    hover_state: false,
                    click_funcs: vec![],

                    focused: false,
                }));
                $name::post_init(v.clone());
                ui.add(Element::$name(v.clone()), auto_free);
                v
            }
        }
    )
}

element! {
    ref ImageRef
    pub struct Image {
        pub texture: String,
        pub width: f64,
        pub height: f64,
        pub colour: (u8, u8, u8, u8),
        pub texture_coords: (f64, f64, f64, f64),
        priv last_texture: String,
        priv last_colour: (u8, u8, u8, u8),
        priv last_texture_coords: (f64, f64, f64, f64),
    }
    builder ImageBuilder {
        hardcode last_texture = "".into(),
        hardcode last_colour = image::TRANSPARENT_IMAGE_COLOUR,
        hardcode last_texture_coords = image::DEFAULT_TEXTURE_COORDS,
        simple texture: String,
        optional colour: (u8, u8, u8, u8) = image::DEFAULT_IMAGE_COLOUR,
        optional texture_coords: (f64, f64, f64, f64) = image::DEFAULT_TEXTURE_COORDS,
        noset width: f64 = |b| b.width.expect("Missing required field width"),
        noset height: f64 = |b| b.height.expect("Missing required field height"),
    }
}

impl ImageBuilder {
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
}

impl UIElement for Image {
    fn draw(
        &mut self,
        renderer: &mut render::Renderer,
        r: &Region,
        sw: f64,
        sh: f64,
        width: f64,
        height: f64,
        delta: f64,
    ) -> &mut [u8] {
        if self.check_rebuild() {
            self.data.clear();
            let texture = render::Renderer::get_texture(renderer.get_textures_ref(), &self.texture);
            let mut element = render::ui::UIElement::new(
                &texture,
                r.x,
                r.y,
                r.w,
                r.h,
                self.texture_coords.0,
                self.texture_coords.1,
                self.texture_coords.2,
                self.texture_coords.3,
            );
            element.r = self.colour.0;
            element.g = self.colour.1;
            element.b = self.colour.2;
            element.a = self.colour.3;
            self.data.extend_from_slice(&element.bytes(width, height));
            self.super_draw(renderer, r, sw, sh, width, height, delta);
            self.last_texture = self.texture.clone();
            self.last_colour = self.colour;
            self.last_texture_coords = self.texture_coords;
        }
        &mut self.data
    }

    fn tick(&mut self, renderer: &mut render::Renderer) {
        self.super_tick(renderer);
    }

    fn get_size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    fn is_dirty(&self) -> bool {
        image::image_visual_state_changed(
            &image::ImageRenderState {
                texture: &self.last_texture,
                colour: self.last_colour,
                texture_coords: self.last_texture_coords,
            },
            &image::ImageRenderState {
                texture: &self.texture,
                colour: self.colour,
                texture_coords: self.texture_coords,
            },
        )
    }
}

element! {
    ref BatchRef
    pub struct Batch {
        pub width: f64,
        pub height: f64,
    }
    builder BatchBuilder {
        noset width: f64 = |b| b.width.expect("Missing required field width"),
        noset height: f64 = |b| b.height.expect("Missing required field height"),
    }
}

impl BatchBuilder {
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
}

impl UIElement for Batch {
    fn draw(
        &mut self,
        renderer: &mut render::Renderer,
        r: &Region,
        sw: f64,
        sh: f64,
        width: f64,
        height: f64,
        delta: f64,
    ) -> &mut [u8] {
        if self.check_rebuild() {
            self.data.clear();
            self.super_draw(renderer, r, sw, sh, width, height, delta);
        }
        &mut self.data
    }

    fn tick(&mut self, renderer: &mut render::Renderer) {
        self.super_tick(renderer);
    }

    fn get_size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    fn is_dirty(&self) -> bool {
        image::batch_visual_state_changed()
    }
}

element! {
    ref TextRef
    pub struct Text {
        pub text: String,
        pub width: f64,
        pub height: f64,
        pub scale_x: f64,
        pub scale_y: f64,
        pub colour: (u8, u8, u8, u8),
        pub rotation: f64,
        priv last_text: String,
        priv last_scale_x: f64,
        priv last_scale_y: f64,
        priv last_colour: (u8, u8, u8, u8),
        priv last_rotation: f64,
    }
    builder TextBuilder {
        hardcode width = 0.0,
        hardcode height = text::TEXT_DEFAULT_HEIGHT,
        hardcode last_text = "".into(),
        hardcode last_scale_x = 0.0,
        hardcode last_scale_y = 0.0,
        hardcode last_colour = text::TEXT_TRANSPARENT_COLOUR,
        hardcode last_rotation = text::TEXT_DEFAULT_ROTATION,
        simple text: String,
        optional scale_x: f64 = text::TEXT_DEFAULT_SCALE,
        optional scale_y: f64 = text::TEXT_DEFAULT_SCALE,
        optional colour: (u8, u8, u8, u8) = text::TEXT_DEFAULT_COLOUR,
        optional rotation: f64 = text::TEXT_DEFAULT_ROTATION,
    }
}

impl UIElement for Text {
    fn draw(
        &mut self,
        renderer: &mut render::Renderer,
        r: &Region,
        sw: f64,
        sh: f64,
        width: f64,
        height: f64,
        delta: f64,
    ) -> &mut [u8] {
        if self.check_rebuild() {
            self.data.clear();

            let mut text = if self.rotation == 0.0 {
                renderer.ui.new_text_scaled(
                    &self.text,
                    r.x,
                    r.y,
                    sw * self.scale_x,
                    sh * self.scale_y,
                    self.colour.0,
                    self.colour.1,
                    self.colour.2,
                )
            } else {
                let c = self.rotation.cos();
                let s = self.rotation.sin();
                let tmpx = r.w / 2.0;
                let tmpy = r.h / 2.0;
                let w = (tmpx * c - tmpy * s).abs();
                let h = (tmpy * c + tmpx * s).abs();
                renderer.ui.new_text_rotated(
                    &self.text,
                    r.x + w - (r.w / 2.0),
                    r.y + h - (r.h / 2.0),
                    sw * self.scale_x,
                    sh * self.scale_y,
                    self.rotation,
                    self.colour.0,
                    self.colour.1,
                    self.colour.2,
                )
            };
            for e in &mut text.elements {
                e.a = self.colour.3;
            }
            self.data.extend_from_slice(&text.bytes(width, height));
            self.super_draw(renderer, r, sw, sh, width, height, delta);

            self.last_text = self.text.clone();
            self.last_colour = self.colour;
            self.last_scale_x = self.scale_x;
            self.last_scale_y = self.scale_y;
            self.last_rotation = self.rotation;
        }
        &mut self.data
    }

    fn tick(&mut self, renderer: &mut render::Renderer) {
        self.super_tick(renderer);
        if self.is_dirty() {
            self.width = renderer.ui.size_of_string(&self.text);
        }
    }

    fn get_size(&self) -> (f64, f64) {
        text::scaled_text_size(text::TextMetrics {
            width: self.width,
            height: self.height,
            scale_x: self.scale_x,
            scale_y: self.scale_y,
        })
    }

    fn is_dirty(&self) -> bool {
        text::text_visual_state_changed(
            (
                &self.last_text,
                self.last_colour,
                self.last_scale_x,
                self.last_scale_y,
                self.last_rotation,
            ),
            (
                &self.text,
                self.colour,
                self.scale_x,
                self.scale_y,
                self.rotation,
            ),
        )
    }
}

element! {
    ref FormattedRef
    pub struct Formatted {
        pub width: f64,
        pub height: f64,
        pub scale_x: f64,
        pub scale_y: f64,
        pub max_width: f64,
        priv text: format::Component,
        priv text_elements: Vec<Element>,
        priv last_text: format::Component,
        priv last_scale_x: f64,
        priv last_scale_y: f64,
        priv last_max_width: f64,
        priv dirty: bool,
    }
    builder FormattedBuilder {
        hardcode width = 0.0,
        hardcode height = text::TEXT_DEFAULT_HEIGHT,
        hardcode text_elements = vec![],
        hardcode last_text = Default::default(),
        hardcode last_scale_x = 0.0,
        hardcode last_scale_y = 0.0,
        hardcode last_max_width = formatting::FORMATTED_NO_WRAP_WIDTH,
        hardcode dirty = true,
        simple text: format::Component,
        optional scale_x: f64 = text::TEXT_DEFAULT_SCALE,
        optional scale_y: f64 = text::TEXT_DEFAULT_SCALE,
        optional max_width: f64 = formatting::FORMATTED_NO_WRAP_WIDTH,
    }
}

impl UIElement for Formatted {
    fn draw(
        &mut self,
        renderer: &mut render::Renderer,
        r: &Region,
        sw: f64,
        sh: f64,
        width: f64,
        height: f64,
        delta: f64,
    ) -> &mut [u8] {
        if self.check_rebuild() {
            self.data.clear();

            self.elements.clear();
            self.text_elements =
                build_formatted_text_elements(renderer, &self.text, self.max_width);

            for e in &self.text_elements {
                if self.needs_rebuild {
                    e.force_rebuild();
                }
                let r = Container::compute_draw_region(e, sw, sh, r);
                let data = e.draw(renderer, &r, sw, sh, width, height, delta);
                self.data.extend_from_slice(&data);
            }
            self.super_draw(renderer, r, sw, sh, width, height, delta);

            self.last_text = self.text.clone();
            self.last_scale_x = self.scale_x;
            self.last_scale_y = self.scale_y;
            self.last_max_width = self.max_width;
            self.dirty = false;
        }
        &mut self.data
    }

    fn tick(&mut self, renderer: &mut render::Renderer) {
        self.super_tick(renderer);
        if self.is_dirty() {
            let (w, h) = Self::compute_size(renderer, &self.text, self.max_width);
            self.width = w;
            self.height = h;
        }
    }

    fn get_size(&self) -> (f64, f64) {
        text::scaled_text_size(text::TextMetrics {
            width: self.width,
            height: self.height,
            scale_x: self.scale_x,
            scale_y: self.scale_y,
        })
    }

    fn is_dirty(&self) -> bool {
        self.dirty
            || self.last_scale_x != self.scale_x
            || self.last_scale_y != self.scale_y
            || self.last_max_width != self.max_width
    }
}

impl Formatted {
    pub fn set_text(&mut self, val: format::Component) {
        self.text = val;
        self.dirty = true;
    }

    pub fn compute_size(
        renderer: &render::Renderer,
        text: &format::Component,
        max_width: f64,
    ) -> (f64, f64) {
        let plan = formatting::plan_formatted_text(text, max_width, |character| {
            renderer.ui.size_of_char(character)
        });
        (plan.width, plan.height)
    }
}

struct FormattedTextElements {
    text: Vec<Element>,
}

impl ElementHolder for FormattedTextElements {
    fn add(&mut self, el: Element, _: bool) {
        self.text.push(el);
    }
}

fn build_formatted_text_elements(
    renderer: &render::Renderer,
    component: &format::Component,
    max_width: f64,
) -> Vec<Element> {
    let plan = formatting::plan_formatted_text(component, max_width, |character| {
        renderer.ui.size_of_char(character)
    });
    let mut elements = FormattedTextElements { text: Vec::new() };
    for run in plan.runs {
        let (red, green, blue) = run.color.to_rgb();
        TextBuilder::new()
            .text(run.text)
            .position(run.x, run.y)
            .colour((red, green, blue, text::TEXT_OPAQUE_ALPHA))
            .create(&mut elements);
    }
    elements.text
}

element! {
    ref ButtonRef
    pub struct Button {
        pub disabled: bool,
        pub width: f64,
        pub height: f64,
        priv hovered: bool,
        priv last_hovered: bool,
        priv last_disabled: bool,
        priv texts: Vec<TextRef>,
    }
    builder ButtonBuilder {
        hardcode hovered = false,
        hardcode last_hovered = false,
        hardcode last_disabled = false,
        hardcode texts = vec![],
        optional disabled: bool = false,
        noset width: f64 = |b| b.width.expect("Missing required field width"),
        noset height: f64 = |b| b.height.expect("Missing required field height"),
    }
}

impl ButtonBuilder {
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
}

impl UIElement for Button {
    fn draw(
        &mut self,
        renderer: &mut render::Renderer,
        r: &Region,
        sw: f64,
        sh: f64,
        width: f64,
        height: f64,
        delta: f64,
    ) -> &mut [u8] {
        if self.check_rebuild() {
            self.data.clear();
            let offset = button::texture_y_offset(button::ButtonState {
                disabled: self.disabled,
                hovered: self.hovered,
            });
            let texture = render::Renderer::get_texture(renderer.get_textures_ref(), "gui/widgets")
                .relative(
                    0.0,
                    offset / button::BUTTON_TEXTURE_ATLAS_SIZE,
                    button::BUTTON_TEXTURE_WIDTH / button::BUTTON_TEXTURE_ATLAS_SIZE,
                    button::BUTTON_TEXTURE_HEIGHT / button::BUTTON_TEXTURE_ATLAS_SIZE,
                );

            self.data.extend(
                render::ui::UIElement::new(
                    &texture,
                    r.x,
                    r.y,
                    4.0 * sw,
                    4.0 * sh,
                    0.0,
                    0.0,
                    2.0 / 200.0,
                    2.0 / 20.0,
                )
                .bytes(width, height),
            );
            self.data.extend(
                render::ui::UIElement::new(
                    &texture,
                    r.x + r.w - 4.0 * sw,
                    r.y,
                    4.0 * sw,
                    4.0 * sh,
                    198.0 / 200.0,
                    0.0,
                    2.0 / 200.0,
                    2.0 / 20.0,
                )
                .bytes(width, height),
            );
            self.data.extend(
                render::ui::UIElement::new(
                    &texture,
                    r.x,
                    r.y + r.h - 6.0 * sh,
                    4.0 * sw,
                    6.0 * sh,
                    0.0,
                    17.0 / 20.0,
                    2.0 / 200.0,
                    3.0 / 20.0,
                )
                .bytes(width, height),
            );
            self.data.extend(
                render::ui::UIElement::new(
                    &texture,
                    r.x + r.w - 4.0 * sw,
                    r.y + r.h - 6.0 * sh,
                    4.0 * sw,
                    6.0 * sh,
                    198.0 / 200.0,
                    17.0 / 20.0,
                    2.0 / 200.0,
                    3.0 / 20.0,
                )
                .bytes(width, height),
            );

            let w = ((r.w / sw) / 2.0) - 4.0;
            self.data.extend(
                render::ui::UIElement::new(
                    &texture.relative(2.0 / 200.0, 0.0, 196.0 / 200.0, 2.0 / 20.0),
                    r.x + 4.0 * sw,
                    r.y,
                    r.w - 8.0 * sw,
                    4.0 * sh,
                    0.0,
                    0.0,
                    w / 196.0,
                    1.0,
                )
                .bytes(width, height),
            );
            self.data.extend(
                render::ui::UIElement::new(
                    &texture.relative(2.0 / 200.0, 17.0 / 20.0, 196.0 / 200.0, 3.0 / 20.0),
                    r.x + 4.0 * sw,
                    r.y + r.h - 6.0 * sh,
                    r.w - 8.0 * sw,
                    6.0 * sh,
                    0.0,
                    0.0,
                    w / 196.0,
                    1.0,
                )
                .bytes(width, height),
            );

            let h = ((r.h / sh) / 2.0) - 5.0;
            self.data.extend(
                render::ui::UIElement::new(
                    &texture.relative(0.0 / 200.0, 2.0 / 20.0, 2.0 / 200.0, 15.0 / 20.0),
                    r.x,
                    r.y + 4.0 * sh,
                    4.0 * sw,
                    r.h - 10.0 * sh,
                    0.0,
                    0.0,
                    1.0,
                    h / 16.0,
                )
                .bytes(width, height),
            );
            self.data.extend(
                render::ui::UIElement::new(
                    &texture.relative(198.0 / 200.0, 2.0 / 20.0, 2.0 / 200.0, 15.0 / 20.0),
                    r.x + r.w - 4.0 * sw,
                    r.y + 4.0 * sh,
                    4.0 * sw,
                    r.h - 10.0 * sh,
                    0.0,
                    0.0,
                    1.0,
                    h / 16.0,
                )
                .bytes(width, height),
            );

            self.data.extend(
                render::ui::UIElement::new(
                    &texture.relative(2.0 / 200.0, 2.0 / 20.0, 196.0 / 200.0, 15.0 / 20.0),
                    r.x + 4.0 * sw,
                    r.y + 4.0 * sh,
                    r.w - 8.0 * sw,
                    r.h - 10.0 * sh,
                    0.0,
                    0.0,
                    w / 196.0,
                    h / 16.0,
                )
                .bytes(width, height),
            );
            self.super_draw(renderer, r, sw, sh, width, height, delta);
            self.last_disabled = self.disabled;
            self.last_hovered = self.hovered;
        }
        &mut self.data
    }

    fn tick(&mut self, renderer: &mut render::Renderer) {
        self.super_tick(renderer);
    }

    fn get_size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    fn is_dirty(&self) -> bool {
        button::visual_state_changed(
            button::ButtonState {
                disabled: self.last_disabled,
                hovered: self.last_hovered,
            },
            button::ButtonState {
                disabled: self.disabled,
                hovered: self.hovered,
            },
        )
    }

    fn post_init(s: Rc<RefCell<Self>>) {
        s.borrow_mut().add_hover_func(move |this, hover, _| {
            this.hovered = hover;
            for text in &this.texts {
                text.borrow_mut().colour.2 = button::hover_text_blue_channel(hover);
            }
            true
        })
    }
}

impl Button {
    pub fn add_text(&mut self, text: TextRef) {
        self.texts.push(text);
    }
}

type SubmitFunc = dyn Fn(&mut TextBox, &mut crate::Game);

element! {
    ref TextBoxRef
    pub struct TextBox {
        pub input: String,
        pub password: bool,
        pub width: f64,
        pub height: f64,
        priv button: Option<ButtonRef>,
        priv text: Option<TextRef>,
        priv was_focused: bool,
        priv cursor_tick: f64,
        priv submit_funcs: Vec<Box<SubmitFunc>>,
    }
    builder TextBoxBuilder {
        hardcode button = None,
        hardcode text = None,
        hardcode was_focused = false,
        hardcode cursor_tick = 0.0,
        hardcode submit_funcs = vec![],
        optional input: String = "".into(),
        optional password: bool = false,
        noset width: f64 = |b| b.width.expect("Missing required field width"),
        noset height: f64 = |b| b.height.expect("Missing required field height"),
    }
}

impl TextBoxBuilder {
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
}

impl UIElement for TextBox {
    fn key_press(
        &mut self,
        game: &mut crate::Game,
        key: VirtualKeyCode,
        down: bool,
        ctrl_pressed: bool,
    ) {
        match (key, down) {
            (VirtualKeyCode::Return, false) => {
                use std::mem;
                let len = self.submit_funcs.len();
                let mut temp = mem::replace(&mut self.submit_funcs, Vec::with_capacity(len));
                for func in &temp {
                    (func)(self, game);
                }
                self.submit_funcs.append(&mut temp);
            }
            // TODO: wasm clipboard pasting, Clipboard API: https://www.w3.org/TR/clipboard-apis/
            #[cfg(not(target_arch = "wasm32"))]
            (VirtualKeyCode::V, true) => {
                if textbox::clipboard_paste_allowed(ctrl_pressed, textbox::ClipboardPath::Supported)
                {
                    let clipboard: Result<ClipboardContext, _> = ClipboardProvider::new();
                    if let Ok(mut clipboard) = clipboard {
                        if let Ok(text) = clipboard.get_contents() {
                            self.input.push_str(&text)
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn key_type(&mut self, _game: &mut crate::Game, c: char) {
        textbox::apply_typed_char(&mut self.input, c);
    }

    fn draw(
        &mut self,
        renderer: &mut render::Renderer,
        r: &Region,
        sw: f64,
        sh: f64,
        width: f64,
        height: f64,
        delta: f64,
    ) -> &mut [u8] {
        if self.check_rebuild() {
            self.data.clear();
            self.cursor_tick = textbox::advance_cursor_tick(self.cursor_tick, delta);
            let mut text = self.transform_input();
            {
                let mut btn = self.button.as_mut().unwrap().borrow_mut();
                btn.width = self.width;
                btn.height = self.height;
                let mut txt = self.text.as_mut().unwrap().borrow_mut();
                if textbox::cursor_is_visible(self.focused, self.cursor_tick) {
                    text.push(textbox::CURSOR_CHAR);
                }
                txt.text = text;
            }
            self.super_draw(renderer, r, sw, sh, width, height, delta);

            self.was_focused = self.focused;
        }
        &mut self.data
    }

    fn tick(&mut self, renderer: &mut render::Renderer) {
        self.super_tick(renderer);
    }

    fn get_size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    fn is_dirty(&self) -> bool {
        self.focused || self.was_focused
    }

    fn post_init(s: Rc<RefCell<Self>>) {
        let mut textbox = s.borrow_mut();
        textbox.button = Some(
            ButtonBuilder::new()
                .position(0.0, 0.0)
                .size(textbox.width, textbox.height)
                .disabled(true)
                .attach(&mut *textbox),
        );
        textbox.text = Some(
            TextBuilder::new()
                .text("")
                .position(5.0, 0.0)
                .draw_index(1)
                .alignment(VAttach::Middle, HAttach::Left)
                .attach(&mut *textbox),
        );
    }
}

impl TextBox {
    pub fn add_submit_func<F: Fn(&mut TextBox, &mut crate::Game) + 'static>(&mut self, f: F) {
        self.submit_funcs.push(Box::new(f));
    }

    fn transform_input(&self) -> String {
        textbox::transformed_input(&self.input, self.password)
    }
}
