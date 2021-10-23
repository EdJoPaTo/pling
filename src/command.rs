use std::io::ErrorKind;
use std::string::ToString;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[doc = include_str!("../docs/command.md")]
pub struct Command {
    pub program: String,

    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub arguments: Vec<String>,
}

impl From<Command> for crate::Notification {
    fn from(command: Command) -> Self {
        Self::Command(command)
    }
}

impl Command {
    #[must_use]
    /// Loads the command to execute on notification from environment variables.
    /// The following variables are used:
    /// - `PLING_COMMAND_PROGRAM`
    /// - `PLING_COMMAND_ARGS`
    ///
    /// When `PLING_COMMAND_PROGRAM` is unset None is returned.
    pub fn from_env() -> Option<Self> {
        let program = std::env::var("PLING_COMMAND_PROGRAM").ok()?;
        let arguments = std::env::var("PLING_COMMAND_ARGS")
            .unwrap_or_default()
            .split(' ')
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        Some(Self { program, arguments })
    }

    /// Execute a command locally.
    /// This can do everything your local system can do.
    ///
    /// The `text` is passed as the last argument.
    ///
    /// # Errors
    ///
    /// When the command isnt found or when the `ExitCode` isn't successful.
    pub fn send_sync(&self, text: &str) -> Result<(), std::io::Error> {
        let output = std::process::Command::new(&self.program)
            .args(&self.arguments)
            .arg(text)
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            Err(std::io::Error::new(
                ErrorKind::Other,
                "Command exited unsuccessfully",
            ))
        }
    }
}

#[test]
fn command_works() {
    let command = Command {
        program: "true".into(),
        arguments: vec![],
    };

    command.send_sync("something").unwrap();
}

#[test]
#[should_panic = "Command exited unsuccessfully"]
fn command_unsuccessful_errors_works() {
    let command = Command {
        program: "false".into(),
        arguments: vec![],
    };

    command.send_sync("something").unwrap();
}
