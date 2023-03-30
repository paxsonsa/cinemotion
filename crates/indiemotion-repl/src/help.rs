use crate::error::*;
use crate::BlockOutput;
use crate::Parameter;
use yansi::Paint;

/// Help entry which gets sent to [HelpViewer](trait.HelpViewer.html) when help for a particular
/// command is requested
#[derive(Debug)]
pub struct HelpEntry {
    /// Command from `help <command>`
    pub command: String,

    /// Parameters defined for the command
    pub parameters: Vec<(String, bool)>,

    /// Help summary for the command
    pub summary: Option<String>,
}

impl HelpEntry {
    pub(crate) fn new(
        command_name: &str,
        parameters: &[Parameter],
        summary: &Option<String>,
    ) -> Self {
        Self {
            command: command_name.to_string(),
            parameters: parameters
                .iter()
                .map(|pd| (pd.name.clone(), pd.required))
                .collect(),
            summary: summary.clone(),
        }
    }
}

/// Struct which gets sent to [HelpViewer](trait.HelpViewer.html) when `help` command is called
pub struct HelpContext {
    /// Application name
    pub app_name: String,

    /// Application version
    pub app_version: String,

    /// Application purpose/description
    pub app_purpose: String,

    /// List of help entries
    pub help_entries: Vec<HelpEntry>,
}

impl HelpContext {
    pub(crate) fn new(
        app_name: &str,
        app_version: &str,
        app_purpose: &str,
        help_entries: Vec<HelpEntry>,
    ) -> Self {
        Self {
            app_name: app_name.into(),
            app_version: app_version.into(),
            app_purpose: app_purpose.into(),
            help_entries,
        }
    }
}

/// Trait to be used if you want your own custom Help output
pub trait HelpViewer {
    /// Called when the plain `help` command is called with no arguments
    fn help_general(&self, context: &HelpContext) -> Result<BlockOutput>;

    /// Called when the `help` command is called with a command argument (i.e., `help foo`).
    /// Note that you won't have to handle an unknown command - it'll be handled in the caller
    fn help_command(&self, entry: &HelpEntry) -> Result<BlockOutput>;
}

/// Default [HelpViewer](trait.HelpViewer.html)
pub struct DefaultHelpViewer;

impl DefaultHelpViewer {
    pub fn new() -> Self {
        Self
    }
}

impl HelpViewer for DefaultHelpViewer {
    fn help_general(&self, context: &HelpContext) -> Result<BlockOutput> {
        let mut block = BlockOutput::default();
        self.help_header(&mut block, context);
        for entry in &context.help_entries {
            block.add_line(format!("{}", entry.command));
            if entry.summary.is_some() {
                block.append(format!(" - {}", entry.summary.as_ref().unwrap()));
            }
        }
        Ok(block)
    }

    fn help_command(&self, entry: &HelpEntry) -> Result<BlockOutput> {
        let mut block = BlockOutput::default();
        if entry.summary.is_some() {
            block.add_line(format!("{}\n", entry.summary.as_ref().unwrap()));
        } else {
            block.add_line("No summary.".to_string());
        }
        block.add_line("".to_string());
        block.add_line(format!("USAGE:"));
        block.add_line(format!("     {}", entry.command));
        for param in &entry.parameters {
            if param.1 {
                block.append(format!(" {}", param.0));
            } else {
                block.append(format!(" [{}]", param.0));
            }
        }

        Ok(block)
    }
}

impl DefaultHelpViewer {
    fn help_header(&self, block: &mut BlockOutput, context: &HelpContext) {
        let header = format!(
            "{} {}: {}",
            context.app_name, context.app_version, context.app_purpose
        );
        let underline = Paint::new(
            std::iter::repeat("-")
                .take(header.len())
                .collect::<String>(),
        );

        block.add_line(format!("{}", header));
        block.add_line(format!("{}", underline));
    }
}
