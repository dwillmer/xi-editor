// Copyright 2017 Google Inc. All rights reserved.
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

//! Structured representation of a plugin's features and capabilities.

use std::path::PathBuf;

use syntax::SyntaxDefinition;

// optional environment variable for debug plugin executables
#[cfg(not(target_os = "fuchsia"))]
static PLUGIN_DIR: &'static str = "XI_PLUGIN_DIR";

// example plugins. Eventually these should be loaded from disk.
#[cfg(not(target_os = "fuchsia"))]
pub fn debug_plugins() -> Vec<PluginDescription> {
    use std::env;
    use self::PluginActivation::*;
    use self::PluginScope::*;
    let plugin_dir = match env::var(PLUGIN_DIR).map(PathBuf::from) {
        Ok(p) => p,
        Err(_) => env::current_exe().unwrap().parent().unwrap().to_owned(),
    };
    print_err!("looking for debug plugins in {:?}", plugin_dir);

    let make_path = |p: &str| -> PathBuf {
        let mut pb = plugin_dir.clone();
        pb.push(p);
        pb
    };

    vec![
        PluginDescription::new("syntect", "0.0", BufferLocal, make_path("xi-syntect-plugin"),
        vec![Autorun]),
        PluginDescription::new("braces", "0.0", BufferLocal, make_path("bracket_example.py"),
        Vec::new()),
        PluginDescription::new("spellcheck", "0.0", BufferLocal, make_path("spellcheck.py"),
        Vec::new()),
        PluginDescription::new("shouty", "0.0", BufferLocal, make_path("shouty.py"),
        Vec::new()),
    ].iter()
        .filter(|desc|{
            if !desc.exec_path.exists() {
                print_err!("missing plugin {} at {:?}", desc.name, desc.exec_path);
                false
            } else {
                true
            }
        })
        .map(|desc| desc.to_owned())
        .collect::<Vec<_>>()
}

#[cfg(target_os = "fuchsia")]
pub fn debug_plugins() -> Vec<PluginDescription> {
    Vec::new()
}

/// Describes attributes and capabilities of a plugin.
///
/// Note: - these will eventually be loaded from manifest files.
#[derive(Debug, Clone)]
pub struct PluginDescription {
    pub name: String,
    pub version: String,
    pub scope: PluginScope,
    // more metadata ...
    /// path to plugin executable
    pub exec_path: PathBuf,
    /// Events that cause this plugin to run
    pub activations: Vec<PluginActivation>,
}

/// `PluginActivation`s represent events that trigger running a plugin.
#[derive(Debug, Clone)]
pub enum PluginActivation {
    /// Always run this plugin, when available.
    Autorun,
    /// Run this plugin if the provided SyntaxDefinition is active.
    #[allow(dead_code)]
    OnSyntax(SyntaxDefinition),
    /// Run this plugin in response to a given command.
    #[allow(dead_code)]
    OnCommand,
}

#[derive(Debug, Clone)]
/// Describes the scope of events a plugin receives.
pub enum PluginScope {
    /// The plugin receives events from multiple buffers.
    Global,
    /// The plugin receives events for a single buffer.
    BufferLocal,
    /// The plugin is launched in response to a command, and receives no
    /// further updates.
    SingleInvocation,
}

#[cfg(not(target_os = "fuchsia"))]
impl PluginDescription {
    fn new<S, P>(name: S, version: S, scope: PluginScope, exec_path: P,
                 activations: Vec<PluginActivation>) -> Self
        where S: Into<String>, P: Into<PathBuf>
    {
        PluginDescription {
            name: name.into(),
            scope: scope,
            version: version.into(),
            exec_path: exec_path.into(),
            activations: activations,
        }
    }

    /// Returns `true` if this plugin is globally scoped, else `false`.
    pub fn is_global(&self) -> bool {
        match self.scope {
            PluginScope::Global => true,
            _ => false,
        }
    }
}
