use std::iter;

use itertools::Itertools;
use tracing::debug;

use crate::{
    api::client::{Action, Artifact, GameArgument, GameRule, JvmArgument, Rule, Value},
    client,
    launcher::{Launcher, Quickplay},
};

pub struct JvmArgsParser<'a> {
    launcher: &'a Launcher,
    manifest: &'a client::manifest::Manifest,
}

impl<'a> JvmArgsParser<'a> {
    #[must_use]
    pub const fn new(launcher: &'a Launcher, manifest: &'a client::manifest::Manifest) -> Self {
        Self { launcher, manifest }
    }

    #[must_use]
    pub fn parse_jvm_args(&self) -> Vec<String> {
        let args = self.manifest.arguments();

        let jvm = args.jvm();
        jvm.iter()
            .map(|arg| match arg {
                JvmArgument::String(arg) => self.parse_java_arg_str(arg),
                JvmArgument::Class(class) => {
                    let passes = class.rules.iter().all(Rule::passes);

                    if !passes {
                        return String::new();
                    };

                    match &class.value {
                        Value::String(s) => self.parse_java_arg_str(&s),
                        Value::StringArray(a) => {
                            a.iter().map(|v| self.parse_java_arg_str(v)).join(" ")
                        }
                    }
                }
            })
            .filter(|s| !s.is_empty())
            .collect()
    }

    fn parse_java_arg_str(&self, arg: &str) -> String {
        arg.replace(
            "${natives_directory}",
            self.launcher
                .libraries_directory()
                .join("natives")
                .to_str()
                .unwrap_or_default(),
        )
        .replace("${launcher_name}", self.launcher.launcher_name())
        .replace("${launcher_version}", self.launcher.launcher_version())
        .replace("${classpath}", &self.get_classpath())
    }

    fn get_classpath(&self) -> String {
        let libraries = self.manifest.libraries();

        libraries
            .iter()
            .filter(|lib| lib.check_rules_passes())
            .flat_map(|lib| self.get_lib_path(lib))
            .chain(iter::once(
                self.launcher.jar_path().to_str().unwrap().to_string(),
            ))
            .join(if cfg!(windows) { ";" } else { ":" })
    }

    fn get_lib_path(&self, lib: &client::library::Library) -> Vec<String> {
        let paths = [
            lib.native_for_current_os(),
            lib.downloads().artifact.as_ref(),
        ];

        paths
            .into_iter()
            .flatten()
            .map(Artifact::path)
            .map(|path| self.launcher.libraries_directory().join(path))
            .filter_map(|path| dunce::canonicalize(path).ok())
            .filter_map(|path| path.to_str().map(ToString::to_string))
            .collect()
    }
}

pub struct MinecraftArgsParser<'a> {
    launcher: &'a Launcher,
    manifest: &'a client::manifest::Manifest,
}

impl<'a> MinecraftArgsParser<'a> {
    #[must_use]
    pub const fn new(launcher: &'a Launcher, manifest: &'a client::manifest::Manifest) -> Self {
        Self { launcher, manifest }
    }

    fn is_excluded_minecraft_arg(arg_str: &str) -> bool {
        let excluded = ["--xuid", "${auth_xuid}", "--clientId", "${clientid}"];

        excluded.contains(&arg_str)
    }

    #[must_use]
    #[tracing::instrument(skip(self))]
    pub fn parse_minecraft_args(&self) -> Vec<String> {
        debug!("Parsing minecraft args");
        self.manifest
            .arguments()
            .game()
            .iter()
            .map(|arg| match arg {
                GameArgument::GameClass(class) => {
                    let passes = class
                        .rules()
                        .iter()
                        .all(|rule| self.minecraft_rule_passes(rule));

                    if !passes {
                        return String::new();
                    };

                    match class.value() {
                        Value::String(s) => self.parse_minecraft_arg_str(s),
                        Value::StringArray(a) => {
                            a.iter().map(|v| self.parse_minecraft_arg_str(v)).join(" ")
                        }
                    }
                }
                GameArgument::String(arg) => self.parse_minecraft_arg_str(arg),
            })
            .filter(|s| !s.is_empty())
            .collect()
    }

    #[tracing::instrument(skip(self))]
    fn parse_minecraft_arg_str(&self, minecraft_arg: &str) -> String {
        debug!("Parsing minecraft arg: {}", minecraft_arg);

        // if excluded return emptry String
        if Self::is_excluded_minecraft_arg(minecraft_arg) {
            return String::new();
        }

        minecraft_arg
            .replace(
                "${auth_player_name}",
                &self
                    .launcher
                    .authentication_details()
                    .auth_details()
                    .username,
            )
            .replace(
                "${version_name}",
                &self.launcher.version_name().replace([' ', ':'], "_"),
            )
            .replace(
                "${game_directory}",
                self.launcher.game_directory().to_str().unwrap_or_default(),
            )
            .replace(
                "${assets_root}",
                self.launcher
                    .assets_directory()
                    .to_str()
                    .unwrap_or_default(),
            )
            .replace("${assets_index_name}", self.launcher.manifest().assets())
            .replace(
                "${auth_uuid}",
                &self.launcher.authentication_details().auth_details().id,
            )
            .replace(
                "${auth_access_token}",
                &self
                    .launcher
                    .authentication_details()
                    .auth_details()
                    .access_token,
            )
            .replace("${user_type}", "msa") // copper only supports MSA
            .replace(
                "${version_type}",
                if self.launcher.is_snapshot() {
                    "snapshot"
                } else {
                    "release"
                },
            )
            .replace(
                "${resolution_width}",
                &self
                    .launcher
                    .custom_resolution()
                    .map(|r| r.width.to_string())
                    .unwrap_or_default(),
            )
            .replace(
                "${resolution_height}",
                &self
                    .launcher
                    .custom_resolution()
                    .as_ref()
                    .map(|r| r.height.to_string())
                    .unwrap_or_default(),
            )
    }

    fn quickplay_check<T: Fn(&Quickplay) -> bool>(&self, x: bool, qp: T) -> bool {
        x == self.launcher.quickplay().is_some_and(qp)
    }

    fn minecraft_rule_passes(&self, rule: &GameRule) -> bool {
        match rule.action() {
            Action::Allow => {
                let features = rule.features();

                let demo_check = features.demo_user().map_or(true, |demo_user| {
                    demo_user == self.launcher.authentication_details().is_demo_user()
                });

                let support_check =
                    features
                        .quick_plays_support()
                        .map_or(true, |quick_plays_support| {
                            quick_plays_support == self.launcher.quickplay().is_some()
                        });

                let quickplay_singleplayer_check =
                    features.quick_play_singleplayer().map_or(true, |x| {
                        self.quickplay_check(x, Quickplay::is_singleplayer)
                    });

                let quickplay_multiplayer_check = features
                    .quick_play_multiplayer()
                    .map_or(true, |x| self.quickplay_check(x, Quickplay::is_multiplayer));

                let quickplay_realms_check = features
                    .quick_play_realms()
                    .map_or(true, |x| self.quickplay_check(x, Quickplay::is_realms));

                let custom_resolution_check = features
                    .custom_resolution()
                    .map_or(true, |x| x == self.launcher.custom_resolution().is_some());

                demo_check
                    && support_check
                    && quickplay_singleplayer_check
                    && quickplay_multiplayer_check
                    && quickplay_realms_check
                    && custom_resolution_check
            }
            Action::Disallow => {
                todo!("disallow rules are not supported yet, as none exist")
            }
        }
    }
}
