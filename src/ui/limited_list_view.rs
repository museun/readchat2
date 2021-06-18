#![cfg_attr(debug_assertions, allow(dead_code,))]

use std::{collections::VecDeque, ops::DerefMut};

use cursive::{
    direction::{Direction, Orientation, Relative},
    event::{AnyCb, Event, EventResult},
    view::{Selector, ViewNotFound},
    Printer, Rect, Vec2, View,
};

pub struct LimitedListView {
    list: VecDeque<Box<dyn View>>,
    heights: VecDeque<usize>,
    max: usize,
    focus: usize,
}

impl LimitedListView {
    pub fn limited_to(max: usize) -> Self {
        Self {
            list: VecDeque::with_capacity(max),
            heights: VecDeque::with_capacity(max),
            max,
            focus: 0,
        }
    }

    pub fn add_child<V>(&mut self, view: V)
    where
        V: cursive::view::IntoBoxedView + 'static,
    {
        let mut view = view.into_boxed_view();
        view.take_focus(Direction::none());

        while self.list.len() >= self.max {
            self.list.pop_front();
            self.heights.pop_front();
        }
        self.list.push_back(view);
        self.heights.push_back(0)
    }

    pub fn child<V>(mut self, view: V) -> Self
    where
        V: cursive::view::IntoBoxedView + 'static,
    {
        self.add_child(view);
        self
    }

    pub fn clear(&mut self) {
        self.list.clear();
        self.heights.clear();
        self.focus = 0
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn children(&mut self) -> &[Box<dyn View>] {
        self.list.make_contiguous();
        let (head, _) = self.list.as_slices();
        head
    }

    pub fn children_mut(&mut self) -> &mut [Box<dyn View>] {
        self.list.make_contiguous();
        let (head, _) = self.list.as_mut_slices();
        head
    }

    pub fn visit_mut<F, R>(&mut self, focused: bool, source: Relative, each: F) -> Vec<R>
    where
        F: FnMut((usize, &mut dyn View)) -> R,
    {
        match source {
            Relative::Front => {
                let start = focused.then(|| self.focus).unwrap_or_default();
                self.list
                    .iter_mut()
                    .map(DerefMut::deref_mut)
                    .enumerate()
                    .skip(start)
                    .map(each)
                    .collect()
            }
            Relative::Back => {
                let end = focused
                    .then(|| self.focus + 1)
                    .unwrap_or_else(|| self.list.len());
                self.list
                    .iter_mut()
                    .map(DerefMut::deref_mut)
                    .enumerate()
                    .rev()
                    .skip(end)
                    .map(each)
                    .collect()
            }
        }
    }

    pub fn move_focus(&mut self, delta: usize, direction: Direction) -> EventResult {
        let _index = if let Some(index) =
            direction.relative(Orientation::Vertical).and_then(|_pos| {
                self.list
                    .iter_mut()
                    .enumerate()
                    .skip(1)
                    .filter_map(|(i, view)| view.take_focus(direction).then(|| i))
                    .take(delta)
                    .last()
                // todo use pos
            }) {
            index
        } else {
            return EventResult::Ignored;
        };

        // TODO use index
        EventResult::Consumed(None)
    }
}

impl View for LimitedListView {
    fn draw(&self, printer: &Printer<'_, '_>) {
        if self.is_empty() {
            return;
        }

        let mut y = 0;
        for (i, (child, &height)) in self.list.iter().zip(&self.heights).enumerate() {
            child.draw(
                &printer
                    .offset((0, y))
                    .cropped((printer.size.x, height))
                    .focused(i == self.focus),
            );
            y += height
        }
    }

    fn layout(&mut self, size: Vec2) {
        let available = size.x;
        self.heights.resize(self.list.len(), 0);

        for (child, height) in self.list.iter_mut().zip(&mut self.heights) {
            *height = child.required_size(size).y;
            child.layout(Vec2::new(available, *height))
        }
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        let vert = self
            .list
            .iter_mut()
            .map(|child| child.required_size(constraint));

        Orientation::Vertical.stack(vert) + (1, 0)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        if self.is_empty() {
            return EventResult::Ignored;
        }

        let y = self
            .heights
            .iter()
            .take(self.heights.len() - self.focus)
            .sum::<usize>();
        let offset = (1, y);

        let result = self.list[self.focus].on_event(event.relativized(offset));
        if result.is_consumed() {
            return result;
        }

        use cursive::event::{Event::*, Key::*};

        match event {
            Key(Up) if self.focus > 0 => self.move_focus(1, Direction::down()),
            Key(Down) if self.focus + 1 < self.len() => self.move_focus(1, Direction::up()),

            Key(PageUp) => self.move_focus(10, Direction::down()),
            Key(PageDown) => self.move_focus(10, Direction::up()),

            Key(Home) | Ctrl(Home) => self.move_focus(usize::MAX, Direction::up()),
            Key(End) | Ctrl(End) => self.move_focus(usize::MAX, Direction::up()),

            Key(Tab) => self.move_focus(1, Direction::front()),
            Shift(Tab) => self.move_focus(1, Direction::back()),

            _ => EventResult::Ignored,
        }
    }

    fn call_on_any<'a>(&mut self, selector: &Selector<'_>, callback: AnyCb<'a>) {
        for view in &mut self.list {
            view.call_on_any(selector, callback)
        }
    }

    fn focus_view(&mut self, selector: &Selector<'_>) -> Result<(), ViewNotFound> {
        self.focus = self
            .list
            .iter_mut()
            .enumerate()
            .find_map(|(i, v)| v.focus_view(selector).ok().map(|_| i))
            .ok_or(ViewNotFound)?;

        Ok(())
    }

    fn important_area(&self, size: Vec2) -> Rect {
        if self.is_empty() {
            return (0, 0).into();
        }

        let available = Vec2::new(size.x.saturating_sub(1), 0);
        self.list[self.focus].important_area(available) + (1, self.focus)
    }
}
