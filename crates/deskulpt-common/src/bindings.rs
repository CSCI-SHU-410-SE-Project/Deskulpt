//! Common utilities for Deskulpt bindings.

use std::collections::BTreeMap;

use specta::datatype::{DataType, Function};
use specta::{NamedType, Type, TypeCollection};

use crate::event::Event;

/// A collection of types, events, and commands to be exposed to the frontend.
///
/// A [`Bindings`] should always be constructed via a [`BindingsBuilder`].
pub struct Bindings {
    /// A specta type collection.
    pub types: TypeCollection,
    /// A mapping from event names to their data types.
    pub events: BTreeMap<&'static str, DataType>,
    /// A mapping from plugin names to their commands.
    pub commands: BTreeMap<&'static str, Vec<Function>>,
}

/// Builder for a [`Bindings`] instance.
#[derive(Default)]
pub struct BindingsBuilder {
    types: TypeCollection,
    events: BTreeMap<&'static str, DataType>,
    commands: BTreeMap<&'static str, fn(&mut TypeCollection) -> Vec<Function>>,
}

impl BindingsBuilder {
    /// Register a type in the collection.
    pub fn typ<T: NamedType>(&mut self) -> &mut Self {
        self.types.register::<T>();
        self
    }

    /// Register an event in the collection.
    pub fn event<T: Event + Type>(&mut self) -> &mut Self {
        let dt = T::reference(&mut self.types, &[]).inner;
        self.events.insert(T::NAME, dt);
        self
    }

    /// Register commands in the collection.
    ///
    /// The argument should be obtained via the [`collect_commands!`] macro.
    pub fn commands(
        &mut self,
        plugin_name: &'static str,
        commands: fn(&mut TypeCollection) -> Vec<Function>,
    ) -> &mut Self {
        self.commands.insert(plugin_name, commands);
        self
    }

    /// Build the [`Bindings`] instance.
    pub fn build(&mut self) -> Bindings {
        let commands = self
            .commands
            .iter_mut()
            .map(|(k, f)| (*k, f(&mut self.types)))
            .collect();

        Bindings {
            types: self.types.clone(),
            events: self.events.clone(),
            commands,
        }
    }
}

/// Used in [`BindingsBuilder::commands`].
pub use specta::function::collect_functions as collect_commands;
