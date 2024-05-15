use std::env;
use atty::{ Stream, is };

#[derive(Debug)]
pub struct SupportsColorResult {
    stdout: Option<ColorSupport>,
    stderr: Option<ColorSupport>,
}

#[derive(Debug)]
struct ColorSupport {
    level: usize,
    has_basic: bool,
    has_256: bool,
    has_16m: bool,
}
#[derive(Debug)]
struct StreamOptions {
    is_tty: bool,
    sniff_flags: bool,
}

impl Default for StreamOptions {
    fn default() -> Self {
        Self {
            is_tty: false,
            sniff_flags: true,
        }
    }
}

struct Environment {
    term: String,
    colorterm: Option<String>,
    teamcity_version: Option<String>,
    ci: Option<String>,
}

impl Environment {
    fn new() -> Self {
        Self {
            term: env::var("TERM").unwrap_or_else(|_| String::from("")),
            colorterm: env::var("COLORTERM").ok(),
            teamcity_version: env::var("TEAMCITY_VERSION").ok(),
            ci: env::var("CI").ok(),
        }
    }

    fn is_ci(&self) -> bool {
        self.ci.is_some()
    }

    fn is_azure_devops_pipeline(&self) -> bool {
        self.ci.as_ref().map_or(false, |ci| ci == "TF_BUILD" && env::var("AGENT_NAME").is_ok())
    }
}

fn has_flag(flag: &str, argv: &Vec<String>) -> bool {
    let flag_without_dashes = flag.trim_start_matches('-');

    argv.iter().any(|arg| {
        let normalized_arg = arg.trim_start_matches('-').to_lowercase();
        normalized_arg == flag_without_dashes
    })
}

fn translate_level(level: usize) -> Option<ColorSupport> {
    if level == 0 {
        return None;
    }

    Some(ColorSupport {
        level,
        has_basic: true,
        has_256: level >= 2,
        has_16m: level >= 3,
    })
}

fn determine_force_color_level_from_flag() -> usize {
    let argv: Vec<String> = env::args().collect();
    let flag_force_color: usize = if
        has_flag("no-color", &argv) ||
        has_flag("no-colors", &argv) ||
        has_flag("color=false", &argv) ||
        has_flag("color=never", &argv)
    {
        0
    } else if
        has_flag("color", &argv) ||
        has_flag("colors", &argv) ||
        has_flag("color=true", &argv) ||
        has_flag("color=always", &argv)
    {
        1
    } else {
        0
    };

    flag_force_color
}

fn determine_force_color_level_from_env() -> Option<usize> {
    if let Ok(force_color) = env::var("FORCE_COLOR") {
        if force_color == "true" {
            return Some(1);
        } else if force_color == "false" {
            return Some(0);
        } else if force_color.is_empty() {
            return Some(1);
        } else if let Ok(level) = force_color.parse::<usize>() {
            if (0..=3).contains(&level) {
                return Some(level);
            }
        }
    }
    None
}

fn determine_color_support_level(stream: Stream, options: StreamOptions) -> usize {
    let mut flag_force_color = None;
    let argv: Vec<String> = env::args().collect();

    if
        has_flag("no-color") ||
        has_flag("no-colors") ||
        has_flag("color=false") ||
        has_flag("color=never")
    {
        flag_force_color = Some(0);
    } else if
        has_flag("color") ||
        has_flag("colors") ||
        has_flag("color=true") ||
        has_flag("color=always")
    {
        flag_force_color = Some(1);
    }

    let no_flag_force_color = env_force_color();
    let force_color = if sniff_flags { flag_force_color } else { no_flag_force_color };

    if let Some(force_color) = force_color {
        return force_color;
    }

    1
}

fn create_supports_color(stream: Stream, options: StreamOptions) -> Option<ColorSupport> {
    let level = determine_color_support_level(stream, options);
    translate_level(level)
}

pub fn get_supports_color_result() -> SupportsColorResult {
    let mut options_std_out = StreamOptions::default();
    options_std_out.is_tty = is(Stream::Stdout);

    let mut options_std_err = StreamOptions::default();
    options_std_err.is_tty = is(Stream::Stderr);

    SupportsColorResult {
        stdout: create_supports_color(Stream::Stdout, options_std_out),
        stderr: create_supports_color(Stream::Stderr, options_std_err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let args: Vec<String> = env::args().collect();
        let result = has_flag("verbose", &vec![String::from("--verbose")]);
        assert!(result);
    }

    #[test]
    fn test_get_supports_color() {
        let result = get_supports_color_result();
        assert!(result.stdout || !result.stdout, "stdout check failed");
        assert!(result.stderr || !result.stderr, "stderr check failed");
    }
}
