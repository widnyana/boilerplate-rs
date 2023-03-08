use envconfig::Envconfig;


#[derive(Envconfig)]
pub struct HTTPConfig {
    #[envconfig(from = "HTTP_HOST")]
    pub Host: string,
    #[envconfig(from = "HTTP_PORT")]
    pub Port: u16,
}

pub struct Config {
    #[envconfig(nested = true)]
    http: HTTPConfig
}
