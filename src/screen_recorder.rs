use std; // Import the signal crate

#[derive(Debug)]
pub struct ScreenRecorder {
    pub is_recording: bool,
    // captures_folder: String,
    output_file: String,
    child: Option<std::process::Child>,
}

impl ScreenRecorder {
    pub fn new() -> Self {
        // Get the home directory
        let home_dir = std::env::var("HOME").expect("$HOME environmental variable is not set!");

        // Create the folder path as a String
        let folder_path = format!("{}/Videos/rustshot-gtk", home_dir);

        // Create folder if not exists
        if let Err(e) = Self::create_folder_if_not_exists(&folder_path) {
            eprintln!("Error creating folder: {}", e);
        }

        // define the output file
        let out_file = format!("{}/out.mkv", folder_path);

        // Return object
        Self {
            is_recording: false,
            // captures_folder: folder_path,
            output_file: out_file,
            child: None,
        }
    }

    pub fn start_record_screen(&mut self, x: i32, y: i32, w: i32, h: i32) {
        // WARNING: I would like to use FFMPEG (a tool to convert video formats, but also it is a
        // way to record the screen.) However, I do not know how to properly use it in wayland. I
        // get errors and the docs were not so useful. I will use instead 'wf-recorder' (a
        // wlroots-based screen recorder).
        //
        // The way it works is
        //  wf-recorder -a -g "x,y wxh" --file="..." -y
        //
        //  where:
        //  -a, --audio [=DEVICE] = Starts recording the screen with audio. DEVICE argument is optional.
        //  -g, --geometry = Selects a specific part of the screen. The format is "x,y WxH".
        //  --file = the output file
        //  -y, --overwrite = Force overwriting the output file without prompting.

        // Build the --geometry string
        let geometry = format!("{},{} {}x{}", x + 1, y + 1, w, h);

        // Execute the `grim` command
        let child = std::process::Command::new("wf-recorder")
            .arg("-a")
            .arg("-g")
            .arg(&geometry)
            .arg("-f")
            .arg(&self.output_file)
            .arg("-y")
            .stdout(std::process::Stdio::piped())
            // .stderr(std::process::Stdio::piped()) // Capture stderr for error messages
            .spawn() // Execute the command
            .expect("Failed to execute wf-recorder");

        self.child = Some(child);
        self.is_recording = true;
    }

    pub fn stop_recording(&mut self) {
        // Exit if it is not recording
        if self.is_recording == false {
            println!("Reconding is not yet started!");
            return;
        }

        // Otherwise, get child
        let child = self.child.as_mut().expect("Reconding is not yet started!");

        // send SIGINT to the child
        nix::sys::signal::kill(
            nix::unistd::Pid::from_raw(child.id() as i32),
            nix::sys::signal::Signal::SIGINT,
        )
        .expect("cannot send ctrl-c");

        // Optionally, wait for the process to exit
        let _ = child.wait().expect("Process wasn't running");
        println!("Process stopped.");

        // stop recording
        self.is_recording = false;
    }

    pub fn get_file_path(&self) -> String {
        return self.output_file.clone();
    }

    fn create_folder_if_not_exists(path: &str) -> std::io::Result<()> {
        // Create the folder if it does not exist
        if !std::fs::metadata(path).is_ok() {
            std::fs::create_dir(path)?;
            println!("Folder created: {}", path);
        }
        // else {
        //     println!("Folder already exists: {}", path);
        // }
        Ok(())
    }
}
