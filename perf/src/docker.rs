use std::process::{exit, Command, Stdio};

pub fn build(tag: &str, docker_file: &str, quiet: bool) {
    let output = Command::new("docker")
        .args(&["build", "-t", tag, "-f", docker_file, "."])
        .stdout(if quiet {
            Stdio::piped()
        } else {
            Stdio::inherit()
        })
        .stderr(Stdio::inherit())
        .output()
        .expect("Could not build docker image");
    if !output.status.success() {
        eprintln!("Could not build docker image");
        exit(output.status.code().unwrap_or(1));
    }
}

pub fn stop(name: &str, quiet: bool) {
    let output = Command::new("docker")
        .args(&["stop", name])
        .stdout(if quiet {
            Stdio::piped()
        } else {
            Stdio::inherit()
        })
        .stderr(Stdio::inherit())
        .output()
        .expect("Could not stop docker container");
    if !output.status.success() {
        eprintln!("Could not stop docker container");
        exit(output.status.code().unwrap_or(1));
    }
}

pub struct Volume {
    pub host: String,
    pub guest: String,
}

pub struct Port {
    pub host: u32,
    pub guest: u32,
}

pub struct RunOptions {
    pub args: Vec<String>,
    pub volumes: Vec<Volume>,
    pub ports: Vec<Port>,
    pub env: Vec<String>,
    pub entry_point: Option<String>,
    pub detach: bool,
    pub container_name: Option<String>,
}

pub fn run(image: &str, options: RunOptions, quiet: bool) {
    let mut run_args: Vec<String> = vec![];
    run_args.push("run".to_string());
    run_args.push("--rm".to_string());
    if options.detach {
        run_args.push("-d".to_string());
    }
    match options.container_name {
        Some(name) => {
            run_args.push("--name".to_string());
            run_args.push(name);
        }
        None => {}
    }
    for volume in options.volumes {
        run_args.push("-v".to_string());
        run_args.push(format!("{}:{}", volume.host, volume.guest));
    }
    for e in options.env {
        if !e.is_empty() {
            run_args.push("-e".to_string());
            run_args.push(e.clone());
        }
    }
    for port in options.ports {
        run_args.push("-p".to_string());
        run_args.push(format!("{}:{}", port.host, port.guest));
    }
    match options.entry_point {
        Some(entry_point) => {
            run_args.push("--entrypoint".to_string());
            run_args.push(entry_point);
        }
        None => {}
    }
    run_args.push(image.to_string());
    for a in options.args {
        run_args.push(a.to_string());
    }
    println!("{:?}", run_args);
    let output = Command::new("docker")
        .args(run_args)
        .stdout(if quiet {
            Stdio::piped()
        } else {
            Stdio::inherit()
        })
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        eprintln!("Failed");
        exit(output.status.code().unwrap_or(1));
    }
}
