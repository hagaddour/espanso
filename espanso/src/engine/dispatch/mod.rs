/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019-2021 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

use anyhow::Result;
use super::{Event, event::keyboard::Key};

mod executor;
mod default;

pub trait Executor {
  fn execute(&self, event: &Event) -> bool;
}

pub trait Dispatcher {
  fn dispatch(&self, event: Event);
}

pub trait TextInjector {
  fn inject_text(&self, text: &str) -> Result<()>;
}

pub trait KeyInjector {
  fn inject_sequence(&self, keys: &[Key]) -> Result<()>;
}

pub fn default<'a>(text_injector: &'a dyn TextInjector, key_injector: &'a dyn KeyInjector) -> impl Dispatcher + 'a {
  default::DefaultDispatcher::new(
    text_injector,
    key_injector,
  )
}