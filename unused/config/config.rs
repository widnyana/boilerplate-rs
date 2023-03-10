use envconfig::Envconfig;


#[derive(Envconfig)]
pub struct HTTPConfig {
    #[envconfig(from = "HTTP_HOST")]
    pub host: String,
    #[envconfig(from = "HTTP_PORT")]
    pub port: u16,
}

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(nested = true)]
    http: HTTPConfig
}
