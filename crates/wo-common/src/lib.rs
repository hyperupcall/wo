mod passwords;
mod repo;

use fork::{daemon, Fork};
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::{thread::sleep, time::Duration};

use nix::{
	sys::wait::waitpid,
	unistd::{fork, ForkResult},
};

pub use passwords::*;
pub use repo::*;

pub struct Defaults {
	pub win_x_width: f32,
	pub win_y_height: f32,
}

pub fn get_defaults() -> Defaults {
	Defaults {
		win_x_width: 720.0,
		win_y_height: 1080.0,
	}
}

pub fn get_bin_file(bin_name: &str) -> PathBuf {
	let dir = Path::new("/storage/ur/storage_home/Docs/Programming/Repositories/default/wo");

	let bin_file = dir.join("target").join("debug").join(bin_name);
	return bin_file;
}

pub fn good_fork2() {
	if let Ok(Fork::Child) = daemon(false, false) {
		Command::new("/usr/bin/engrampa")
			.output()
			.expect("failed to execute process");
	}
}

pub fn good_fork() {
	unsafe {
		match fork().expect("Failed to fork process") {
			ForkResult::Parent { child } => {
				println!("Try to kill me to check if the target process will be killed");

				// Do not forget to wait for the fork in order to prevent it from becoming a zombie!!!
				waitpid(Some(child), None).unwrap();

				// You have 120 seconds to kill the process :)
				sleep(Duration::from_secs(5));
			}

			ForkResult::Child => {
				nix::unistd::setsid().unwrap();

				// replace with your executable
				Command::new("/usr/bin/engrampa")
					.spawn()
					.expect("failed to spawn the target process");
				exit(0);
			}
		}
	}
}

pub fn take_out() {
	nix::unistd::setsid().unwrap();
}

pub fn fork_gui_foreground(command: &str, args: Vec<String>) {
	unsafe {
		match fork().expect("Failed to fork process") {
			ForkResult::Parent { child } => {
				println!("Child PID: {}", child);

				// Prevent Zombies
				waitpid(Some(child), None).unwrap();

				// You have 120 seconds to kill the process :)
				// sleep(Duration::from_secs(5));
			}

			ForkResult::Child => {
				nix::unistd::setsid().unwrap();

				Command::new(command)
					.args(args)
					.spawn()
					.expect("Failed to spawn the target process");
				exit(0);
			}
		}
	}
}
