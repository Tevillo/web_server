use std::process::Command;

pub fn manipulate_server(s: Server, a: Action) -> Result<String, MyErr> {
    if s == Server::Other || a == Action::Other {
        return Err(MyErr::NotImplemented);
    }

    return match a {
        Action::Close => {
            s.kill();
            Ok(format!("{} closed", s.display()))
        },
        Action::Restart => {
            s.kill();
            start_server(&s, &a)
        }
        Action::Start => start_server(&s, &a),
        _ => Err(MyErr::NotImplemented),
    }
}

fn start_server(s: &Server, a: &Action) -> Result<String, MyErr> {
    let cmd = format!(
         // inital new line will clear terminal, last presses enter
            "\ncd {} && bash -c \"exec -a {} sh run.sh\"\n",
            s.dir(),
            s.display()
        );

    check_screen(&s)?; // Creates Screen if one does not exist
    return match Command::new("screen")
        .arg("-r")
        .arg(s.screen())
        .arg("-X")
        .arg("stuff") //needed for inserting into shell
        .arg(cmd)
        .output()
    {
        Ok(_) => Ok(format!("{} {} Succesfully", s.display(), a.display())),
        Err(_) => Err(MyErr::BuildErr),
    };
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

#[derive(Debug)]
pub enum MyErr {
    BuildErr,
    ScreenErr,
    NotImplemented,
}

#[derive(PartialEq, Debug)]
pub enum Server {
    Bedrock,
    Java,
    Other,
}

#[derive(PartialEq, Debug)]
pub enum Action {
    Start,
    Restart,
    Close,
    Other,
}

impl Server {
    fn screen(&self) -> &str {
        match self {
            Server::Bedrock => "bedrock_server",
            Server::Java => "java_server",
            Server::Other => "other_server",
        }
    }
    pub fn display(&self) -> &str {
        match self {
            Server::Bedrock => "mc_bedrock",
            Server::Java => "mc_java",
            Server::Other => "other",
        }
    }
    fn dir(&self) -> &str {
        match self {
            Server::Bedrock => "~/bedrock_server",
            Server::Java => "~/minecraft_server/main",
            Server::Other => "~",
        }
    }
    pub fn is_online(&self) -> bool {
        let out = Command::new("pgrep").arg("-if").arg(format!("{} run.sh", self.display())).output().unwrap().stdout;
        let out_string = String::from_utf8(out).unwrap_or_default();
        return !out_string.is_empty() // True if exists, false if doesn't
    }
    fn kill(&self) {
        let _ = Command::new("pkill").arg("-if").arg(format!("{} run.sh", self.display())).output();
    }
}

impl Action {
    fn display(&self) -> &str {        
        match self {
            Action::Start => "Started",
            Action::Restart => "Restarted",
            Action::Close => "Closed",
            Action::Other => "other",
        }
    }
}
