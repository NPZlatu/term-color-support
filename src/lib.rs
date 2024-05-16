use std::env;

use atty::{ Stream, is };
use os_info;

#[derive(Debug)]
enum ColorLevel {
    NoColor,
    Basic,
    Colors256,
    TrueColor,
}

impl ColorLevel {
    fn from_u32(level: u32) -> Option<ColorLevel> {
        match level {
            0 => Some(ColorLevel::NoColor),
            1 => Some(ColorLevel::Basic),
            2 => Some(ColorLevel::Colors256),
            3 => Some(ColorLevel::TrueColor),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct OutputColorSupport {
    stdout_color_support: Option<ColorLevel>,
    stderr_color_support: Option<ColorLevel>,
}

impl OutputColorSupport {
    fn new(
        stdout_color_support: Option<ColorLevel>,
        stderr_color_support: Option<ColorLevel>
    ) -> Self {
        OutputColorSupport { stdout_color_support, stderr_color_support }
    }
}

#[derive(Debug)]
struct Environment {
    term: String,
    colorterm: Option<String>,
    teamcity_version: Option<String>,
    ci: Option<String>,
    os_release: String,
    term_program: Option<String>,
    term_program_version: String,
}

impl Environment {
    fn new() -> Self {
        let binding = os_info::get();
        let os_release = binding.version();

        Self {
            term: std::env::var("TERM").unwrap_or_else(|_| String::from("")),
            colorterm: std::env::var("COLORTERM").ok(),
            teamcity_version: std::env::var("TEAMCITY_VERSION").ok(),
            ci: std::env::var("CI").ok(),
            os_release: os_release.to_string(),
            term_program: std::env::var("TERM_PROGRAM").ok(),
            term_program_version: std::env
                ::var("TERM_PROGRAM_VERSION")
                .unwrap_or_else(|_| String::from("")),
        }
    }

    fn get_os_release_parts(&self) -> Vec<u32> {
        self.os_release
            .split('.')
            .map(|part| part.parse().unwrap_or(0))
            .collect()
    }

    fn get_term_program_version_major(&self) -> Option<u32> {
        self.term_program_version
            .split('.')
            .next()
            .and_then(|major| major.parse().ok())
    }

    fn resolve_color_level(&self) -> ColorLevel {
        if self.term == "dumb" {
            return ColorLevel::NoColor;
        }

        if cfg!(windows) {
            let release_parts = self.get_os_release_parts();
            if release_parts[0] >= 10 && release_parts[2] >= 10_586 {
                return if release_parts[2] >= 14_931 {
                    ColorLevel::TrueColor
                } else {
                    ColorLevel::Colors256
                };
            }
            return ColorLevel::Basic;
        }

        if let Some(ci) = &self.ci {
            if ci == "TF_BUILD" && std::env::var("AGENT_NAME").is_ok() {
                return ColorLevel::Basic;
            }
            return ColorLevel::NoColor;
        }

        if let Some(teamcity_version) = &self.teamcity_version {
            if teamcity_version.starts_with("9.") || teamcity_version.starts_with(char::is_numeric) {
                return ColorLevel::Basic;
            }
            return ColorLevel::NoColor;
        }

        if let Some(colorterm) = &self.colorterm {
            if colorterm == "truecolor" {
                return ColorLevel::TrueColor;
            }
        }

        if self.term == "xterm-kitty" {
            return ColorLevel::TrueColor;
        }

        if let Some(term_program) = &self.term_program {
            if let Some(version_major) = self.get_term_program_version_major() {
                match term_program.as_str() {
                    "iTerm.app" => {
                        return if version_major >= 3 {
                            ColorLevel::TrueColor
                        } else {
                            ColorLevel::Colors256
                        };
                    }
                    "Apple_Terminal" => {
                        return ColorLevel::Colors256;
                    }
                    _ => {}
                }
            }
        }

        if self.term.ends_with("-256color") {
            return ColorLevel::Colors256;
        }

        if
            self.term.starts_with("screen") ||
            self.term.starts_with("xterm") ||
            self.term.starts_with("vt100") ||
            self.term.starts_with("vt220") ||
            self.term.starts_with("rxvt") ||
            self.term.contains("color") ||
            self.term.contains("ansi") ||
            self.term.contains("cygwin") ||
            self.term.contains("linux")
        {
            return ColorLevel::Basic;
        }

        if let Some(colorterm) = &self.colorterm {
            if !colorterm.is_empty() {
                return ColorLevel::Basic;
            }
        }

        ColorLevel::NoColor
    }
}

struct StreamOptions {
    is_tty: bool,
    sniff_flags: bool,
}

impl StreamOptions {
    fn new() -> Self {
        StreamOptions {
            is_tty: false,
            sniff_flags: true,
        }
    }
}

fn has_flag(flag: &str) -> bool {
    let args: Vec<String> = std::env::args().collect();

    let flag_without_dashes = flag.trim_start_matches('-');

    args.iter().any(|arg| {
        let normalized_arg = arg.trim_start_matches('-').to_lowercase();
        normalized_arg == flag_without_dashes
    })
}

fn get_force_color_level_from_env() -> Option<ColorLevel> {
    if let Ok(force_color) = std::env::var("FORCE_COLOR") {
        if force_color == "true" {
            return Some(ColorLevel::Basic);
        }
        if force_color == "false" {
            return Some(ColorLevel::NoColor);
        }
        if force_color.is_empty() {
            return Some(ColorLevel::Basic);
        }
        if let Ok(level) = force_color.parse::<u32>() {
            return ColorLevel::from_u32(level);
        }
    }
    None
}

fn get_color_level_from_flag() -> ColorLevel {
    let args: Vec<String> = std::env::args().collect();
    if
        has_flag("no-color") ||
        has_flag("no-colors") ||
        has_flag("color=false") ||
        has_flag("color=never")
    {
        ColorLevel::NoColor
    } else if
        has_flag("color") ||
        has_flag("colors") ||
        has_flag("color=true") ||
        has_flag("color=always")
    {
        ColorLevel::Basic
    } else {
        ColorLevel::NoColor
    }
}

fn get_color_level(has_stream: bool, options: StreamOptions) -> Option<ColorLevel> {
    let force_color_level_from_env = get_force_color_level_from_env();
    let mut color_level_from_flag: ColorLevel = ColorLevel::NoColor;

    if force_color_level_from_env.is_none() {
        color_level_from_flag = get_color_level_from_flag();
    }

    let force_color = if options.sniff_flags == true {
        Some(color_level_from_flag)
    } else {
        force_color_level_from_env
    };

    if force_color.is_some() {
        return force_color;
    }

    if options.sniff_flags == true {
        if has_flag("color=16m") || has_flag("color=full") || has_flag("color=truecolor") {
            return Some(ColorLevel::TrueColor);
        }
        if has_flag("color=256") {
            return Some(ColorLevel::Colors256);
        }
    }

    if has_stream && !options.is_tty && force_color.is_none() {
        return Some(ColorLevel::NoColor);
    }

    let environment = Environment::new();
    Some(environment.resolve_color_level())
}

fn main() {
    let stdout_is_tty = is(Stream::Stdout);
    let stderr_is_tty = is(Stream::Stderr);

    let output = OutputColorSupport::new(
        get_color_level(stdout_is_tty, StreamOptions::new()),
        get_color_level(stderr_is_tty, StreamOptions::new())
    );

    println!("Ouput {:?}", output);
}
