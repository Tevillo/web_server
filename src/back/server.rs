use std::process::Command;

const TOLERABLE: u32 = 24;
const COOLDOWN_TIME: u64 = 60 * 5;

pub fn manipulate_server(s: Server, a: Action) -> Result<String, MyErr> {
    if s == Server::Other || a == Action::Other {
        return Err(MyErr::NotImplemented);
    }

    match a {
        Action::Close => {
            if !s.is_cooling_down() {
                return Err(MyErr::CooldownErr);
            }
            s.kill();
            Ok(format!("{} closed", s.display()))
        }
        Action::Start => {
            s.kill();
            start_server(&s, &a)
        }
        _ => Err(MyErr::NotImplemented),
    }
}

fn start_server(s: &Server, a: &Action) -> Result<String, MyErr> {
    if s.overlaps() {
        return Err(MyErr::PortErr);
    }

    if !ram_space(&s) {
        return Err(MyErr::RamErr);
    }

    let cmd = format!(
        // inital new line will clear terminal, last presses enter
        "\ncd {} && bash -c \"exec -a {} sh run.sh\"\n",
        s.dir(),
        s.display()
    );

    check_screen(s)?; // Creates Screen if one does not exist
    match Command::new("screen")
        .arg("-r")
        .arg(s.screen())
        .arg("-X")
        .arg("stuff") //needed for inserting into shell
        .arg(cmd)
        .output()
    {
        Ok(_) => Ok(format!("{} {} Succesfully", s.display(), a.display())),
        Err(_) => Err(MyErr::BuildErr),
    }
}

fn check_screen(s: &Server) -> Result<(), MyErr> {
    let out = Command::new("screen").arg("-ls").output().unwrap().stdout;
    let out_string = String::from_utf8(out).unwrap_or_default();
    if !(out_string.contains(s.screen())) {
        // Creates screen if it does not exist
        match Command::new("screen").arg("-dmS").arg(s.screen()).output() {
            Ok(_) => return Ok(()),
            Err(_) => return Err(MyErr::ScreenErr),
        }
    }
    Ok(())
}

fn ram_space(new: &Server) -> bool {
    let servers = get_all_servers();
    let mut total_in_use = 0;
    for s in servers {
        if s.is_online() {
            total_in_use += s.ram();
        }
    }
    println!("Total RAM in use: {}", total_in_use);
    !(total_in_use + new.ram() > TOLERABLE)
}

fn get_all_servers() -> Vec<Server> {
    return vec![
        Server::MinecraftBedrock,
        Server::MinecraftVanilla,
        Server::MinecraftAllTheMods,
    ];
}

#[derive(Debug)]
pub enum MyErr {
    BuildErr,
    CooldownErr,
    ScreenErr,
    PortErr,
    RamErr,
    NotImplemented,
}

impl MyErr {
    pub fn display(&self) -> String {
        match self {
            MyErr::BuildErr => String::from("Build error"),
            MyErr::CooldownErr => {
                String::from("Cannot deactivate server within cooldown period of 5 minutes")
            }
            MyErr::ScreenErr => String::from("Error Creating Screen"),
            MyErr::PortErr => String::from("Port is already in use"),
            MyErr::RamErr => String::from("Not enough RAM available on machine"),
            MyErr::NotImplemented => String::from("Not implemented"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Server {
    MinecraftBedrock,
    MinecraftVanilla,
    MinecraftAllTheMods,
    Other,
}

#[derive(PartialEq, Debug)]
pub enum Action {
    Start,
    Close,
    Other,
}

impl Server {
    fn screen(&self) -> &str {
        match self {
            Server::MinecraftBedrock => "bedrock_server",
            Server::MinecraftVanilla => "java_server",
            Server::MinecraftAllTheMods => "all_the_mods_server",
            Server::Other => "other_server",
        }
    }
    pub fn display(&self) -> &str {
        match self {
            Server::MinecraftBedrock => "mc_bedrock",
            Server::MinecraftVanilla => "mc_java",
            Server::MinecraftAllTheMods => "mc_all_the_mods",
            Server::Other => "other",
        }
    }
    fn dir(&self) -> &str {
        match self {
            Server::MinecraftBedrock => "~/servers/bedrock",
            Server::MinecraftVanilla => "~/servers/vanilla/main",
            Server::MinecraftAllTheMods => "~/servers/all_the_mods",
            Server::Other => "~/servers/other_server",
        }
    }
    pub fn port(&self) -> i32 {
        match self {
            Server::MinecraftBedrock => 25565,
            Server::MinecraftVanilla => 25565,
            Server::MinecraftAllTheMods => 25566,
            Server::Other => -1,
        }
    }
    pub fn image(&self) -> &str {
        match self {
            Server::MinecraftBedrock => "/public/images/minecraft.png",
            Server::MinecraftVanilla => "/public/images/minecraft.png",
            Server::MinecraftAllTheMods => "/public/images/all_the_mods.png",
            Server::Other => "/public/images/error.png",
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Server::MinecraftBedrock => "Bedrock",
            Server::MinecraftVanilla => "Java",
            Server::MinecraftAllTheMods => "All The Mods",
            Server::Other => "Other",
        }
    }
    fn overlaps(&self) -> bool {
        match self {
            //Server::MinecraftAllTheMods => return !Server::MinecraftBedrock.is_online(), // Example for same ports
            _ => false,
        }
    }
    fn ram(&self) -> u32 {
        match self {
            Server::MinecraftBedrock => 8,
            Server::MinecraftVanilla => 12,
            Server::MinecraftAllTheMods => 12,
            Server::Other => 0,
        }
    }
    pub fn is_online(&self) -> bool {
        let out = Command::new("pgrep")
            .arg("-if")
            .arg(format!("{} run.sh", self.display()))
            .output()
            .unwrap()
            .stdout;
        let out_string = String::from_utf8(out).unwrap_or_default();
        !out_string.is_empty() // True if exists, false if doesn't
    }
    fn is_cooling_down(&self) -> bool {
        // False if cooling down, True if ready
        let out = Command::new("pgrep")
            .arg("-if")
            .arg(format!("{} run.sh", self.display()))
            .output()
            .unwrap()
            .stdout;
        let out_string = String::from_utf8(out).unwrap_or_default();
        if out_string.is_empty() {
            false // If not running, not cooling down
        } else {
            let pid = out_string.trim().parse::<u32>().unwrap_or_default();
            let start_time = Command::new("ps")
                .arg("-p")
                .arg(pid.to_string())
                .arg("-o")
                .arg("etimes=")
                .output()
                .unwrap()
                .stdout;
            let start_minute = String::from_utf8(start_time).unwrap_or_default();
            let m = start_minute.trim().parse::<u64>().unwrap_or_default();
            m > COOLDOWN_TIME
        }
    }
    fn kill(&self) {
        let _ = Command::new("pkill")
            .arg("-if")
            .arg(format!("{} run.sh", self.display()))
            .output();
    }
}

impl Action {
    fn display(&self) -> &str {
        match self {
            Action::Start => "Started",
            Action::Close => "Closed",
            Action::Other => "other",
        }
    }
}
