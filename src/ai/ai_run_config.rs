use crate::exec::{DryMode, ExecRunConfig};

#[derive(Debug, Clone)]
pub struct AiRunConfig {
	verbose: bool,
	dry_mode: DryMode,
}

impl Default for AiRunConfig {
	fn default() -> Self {
		Self {
			verbose: false,
			dry_mode: DryMode::None,
		}
	}
}

impl AiRunConfig {
	pub fn verbose(&self) -> bool {
		self.verbose
	}
	pub fn dry_mode(&self) -> DryMode {
		self.dry_mode.clone()
	}
}

// region:    --- Froms

impl From<ExecRunConfig> for AiRunConfig {
	fn from(exec_run_config: ExecRunConfig) -> Self {
		Self {
			verbose: exec_run_config.verbose(),
			dry_mode: exec_run_config.dry_mode(),
		}
	}
}

impl From<&ExecRunConfig> for AiRunConfig {
	fn from(exec_run_config: &ExecRunConfig) -> Self {
		Self {
			verbose: exec_run_config.verbose(),
			dry_mode: exec_run_config.dry_mode(),
		}
	}
}

// endregion: --- Froms
