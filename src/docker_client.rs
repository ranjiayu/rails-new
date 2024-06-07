use std::process::{Command, Stdio};

pub struct DockerClient {}

impl DockerClient {
    pub fn build_image(
        ruby_version: &str,
        rails_version: &str,
        user_id: Option<u32>,
        group_id: Option<u32>,
    ) -> Command {
        let mut command = Command::new("docker");

        command.arg("build");

        Self::set_build_arg(&mut command, "RUBY_VERSION", ruby_version);
        Self::set_build_arg(&mut command, "RAILS_VERSION", rails_version);

        if let Some(id) = user_id {
            Self::set_build_arg(&mut command, "USER_ID", &id.to_string())
        }
        if let Some(id) = group_id {
            Self::set_build_arg(&mut command, "GROUP_ID", &id.to_string())
        }

        command.arg("-t");

        Self::set_image_name(&mut command, ruby_version, rails_version);

        command.arg("-").stdin(Stdio::piped());

        command
    }

    pub fn run_image(ruby_version: &str, rails_version: &str, args: Vec<String>) -> Command {
        let mut command = Self::run();

        Self::set_workdir(&mut command);
        Self::set_image_name(&mut command, ruby_version, rails_version);
        Self::set_rails_new(&mut command, args);

        command
    }

    pub fn get_help(ruby_version: &str, rails_version: &str) -> Command {
        let mut command = Self::run();

        Self::set_image_name(&mut command, ruby_version, rails_version);
        Self::set_rails_new(&mut command, vec!["--help".to_string()]);

        command
    }

    fn run() -> Command {
        let mut command = Command::new("docker");

        command.args(["run", "--rm"]);

        command
    }

    fn set_build_arg(command: &mut Command, key: &str, value: &str) {
        command.args(["--build-arg", &format!("{}={}", key, value)]);
    }

    fn set_workdir(command: &mut Command) {
        let binding = std::env::current_dir().unwrap();
        let current_dir = binding.to_str().unwrap();
        // On Windows platform, make path like "C:\foo\bar" to "/C/foo/bar"
        let current_dir_windows = current_dir.replace(":", "").replace("\\", "/");
        let current_dir_windows = format!("/{}", current_dir_windows);
        command
            .arg("-v")
            .arg(format!("{}:{}", current_dir_windows, current_dir_windows))
            .arg("-w")
            .arg(format!("{}", current_dir_windows));
    }

    fn set_image_name(command: &mut Command, ruby_version: &str, rails_version: &str) {
        command.arg(format!("rails-new-{}-{}", ruby_version, rails_version));
    }

    fn set_rails_new(command: &mut Command, args: Vec<String>) {
        command.args(["rails", "new"]).args(args);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env::current_dir, ffi::OsStr};

    #[test]
    fn build_image() {
        let command = DockerClient::build_image("3.2.3", "7.1.3", None, None);

        assert_eq!(command.get_program(), "docker");

        let args: Vec<&OsStr> = command.get_args().collect();

        assert_eq!(
            args,
            &[
                "build",
                "--build-arg",
                "RUBY_VERSION=3.2.3",
                "--build-arg",
                "RAILS_VERSION=7.1.3",
                "-t",
                "rails-new-3.2.3-7.1.3",
                "-",
            ]
        );
    }

    #[test]
    fn build_image_with_user_id() {
        let command = DockerClient::build_image("3.2.3", "7.1.3", Some(1000), None);

        assert_eq!(command.get_program(), "docker");

        let args: Vec<&OsStr> = command.get_args().collect();

        assert_eq!(
            args,
            &[
                "build",
                "--build-arg",
                "RUBY_VERSION=3.2.3",
                "--build-arg",
                "RAILS_VERSION=7.1.3",
                "--build-arg",
                "USER_ID=1000",
                "-t",
                "rails-new-3.2.3-7.1.3",
                "-",
            ]
        );
    }

    #[test]
    fn build_image_with_group_id() {
        let command = DockerClient::build_image("3.2.3", "7.1.3", None, Some(1000));

        assert_eq!(command.get_program(), "docker");

        let args: Vec<&OsStr> = command.get_args().collect();

        assert_eq!(
            args,
            &[
                "build",
                "--build-arg",
                "RUBY_VERSION=3.2.3",
                "--build-arg",
                "RAILS_VERSION=7.1.3",
                "--build-arg",
                "GROUP_ID=1000",
                "-t",
                "rails-new-3.2.3-7.1.3",
                "-",
            ]
        );
    }

    #[test]
    fn run_image() {
        let command = DockerClient::run_image("3.2.3", "7.1.3", vec!["my_app".to_string()]);

        assert_eq!(command.get_program(), "docker");

        let binding = current_dir().unwrap();
        let current_dir = binding.to_str().unwrap();

        let args: Vec<&OsStr> = command.get_args().collect();

        assert_eq!(
            args,
            &[
                "run",
                "--rm",
                "-v",
                &format!("{}:{}", current_dir, current_dir),
                "-w",
                current_dir,
                "rails-new-3.2.3-7.1.3",
                "rails",
                "new",
                "my_app",
            ]
        );
    }

    #[test]
    fn get_help() {
        let command = DockerClient::get_help("3.2.3", "7.1.3");

        assert_eq!(command.get_program(), "docker");

        let args: Vec<&OsStr> = command.get_args().collect();

        assert_eq!(
            args,
            &[
                "run",
                "--rm",
                "rails-new-3.2.3-7.1.3",
                "rails",
                "new",
                "--help",
            ]
        );
    }
}
