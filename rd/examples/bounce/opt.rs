pub use structopt::StructOpt;

/// Bouncer CLI help
#[derive(StructOpt, Debug)]
#[structopt(name = "Bouncer")]
pub struct Opt {
    /// Set velocity
    #[structopt(short = "v", long, default_value = "1.0")]
    pub velocity: f32,

    /// Set window width
    #[structopt(long, default_value = "700")]
    pub ww: u16,

    /// Set window height
    #[structopt(long, default_value = "700")]
    pub wh: u16,

    /// Set target frames per second
    #[structopt(long, default_value = "60")]
    pub fps: u16,

    /// Set node number
    #[structopt(short = "n", long, default_value = "40")]
    pub node_number: u16,
}
