pub struct Server {
    adress: (String, u16),
    log_level: String,
    proxy_target: Option<String>,
}

impl Server {
    pub fn new(adress: (String, u16), log_level: String, proxy_target: Option<String>) -> Self {
        Server {
            adress,
            log_level,
            proxy_target,
        }
    }

    pub fn adress(&self) -> (String, u16) {
        self.adress.clone()
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn proxy_target(&self) -> Option<&str> {
        self.proxy_target.as_deref()
    }
}

impl std::fmt::Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ascii(self))
    }
}

pub fn from_env() -> Server {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8111".to_string())
        .parse()
        .unwrap_or(8111);
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let proxy_target = std::env::var("PROXY_TARGET").ok();
    Server::new(("0.0.0.0".to_string(), port), log_level, proxy_target)
}

fn ascii(server: &Server) -> String {
    let (root, port) = server.adress();

    let d = format!("http://{root}:{port}");
    format!(
        "
        __| |_________________| |__
        __   _________________   __
          | |                 | |       Catch your http requests
          | | ╔═╗┌─┐┌┬┐┌─┐┬ ┬ | |
          | | ║  ├─┤ │ │  ├─┤ | |       https://github.com/SilenLoc/catch
          | | ╚═╝┴ ┴ ┴ └─┘┴ ┴ | |
        __| |_________________| |__     {d}
        __   _________________   __
          | |                 | |
        ",
    )
}
