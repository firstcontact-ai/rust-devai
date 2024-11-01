use crate::agent::get_solo_and_target_path;
use crate::cli::NewSoloArgs;
use crate::exec::support::{first_file_from_dirs, open_vscode};
use crate::hub::get_hub;
use crate::init::{DEVAI_NEW_CUSTOM_SOLO_AGENT_DIR, DEVAI_NEW_DEFAULT_SOLO_AGENT_DIR};
use crate::Result;
use simple_fs::ensure_file_dir;
use std::path::Path;

/// exec for the New command
pub async fn exec_new_solo(new_config: impl Into<NewSoloConfig>) -> Result<()> {
	let hub = get_hub();

	let new_config = new_config.into();

	// TODO: support --template template_name
	let template_file = first_file_from_dirs(
		&[DEVAI_NEW_CUSTOM_SOLO_AGENT_DIR, DEVAI_NEW_DEFAULT_SOLO_AGENT_DIR],
		"default.devai", // for now, just look for default.devai
	)
	.ok()
	.flatten()
	.ok_or("solo agent template 'default.devai' not found")?;

	let solo_file_path = if new_config.path.ends_with(".devai") {
		new_config.path
	} else {
		format!("{}.devai", new_config.path)
	};
	let solo_file_path = Path::new(&solo_file_path);

	// if it does not exist, we create
	if !solo_file_path.exists() {
		ensure_file_dir(solo_file_path)?;
		std::fs::copy(template_file.path(), solo_file_path)?;

		hub.publish(format!(
			"-> New solo file created: {}",
			solo_file_path.to_string_lossy()
		))
		.await;
	}
	// If already exists, we publish a message, but we do not break
	else {
		hub.publish(format!(
			"-! Solo agent file '{}' already exists.",
			solo_file_path.to_string_lossy()
		))
		.await;
	}

	// We open no matter what
	if new_config.open {
		open_vscode(solo_file_path).await;

		// open the target file if exists
		let (_, target_path) = get_solo_and_target_path(solo_file_path)?;
		if target_path.path().exists() {
			open_vscode(target_path.path()).await;
		}
	}

	Ok(())
}

// region:    --- NewConfig

#[derive(Debug)]
pub struct NewSoloConfig {
	/// The path of the solo .devai of the target file (in this case .devai will be added)
	pub path: String,

	/// If the file(s) needs to be open (via code of vscode for now)
	pub open: bool,
}

impl From<NewSoloArgs> for NewSoloConfig {
	fn from(args: NewSoloArgs) -> Self {
		NewSoloConfig {
			path: args.path,
			open: args.open,
		}
	}
}

// endregion: --- NewConfig
