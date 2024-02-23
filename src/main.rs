use clap;
use clap::Parser;

/// Loud typing
///
/// Trigger noises when you type
#[clap(version = "1.0", author, about, long_about = None)]
struct Cli {
    /// One or multiple audio files or one or multiple directories containing audio files
    #[clap(name = "INPUT", default_value = "./sounds/minecraft/villagers")]
    input: Vec<PathBuf>,

    /// Play sounds in random order
    #[clap(short, long, default_value = "false")]
    random: bool,

    /// Play sounds with a random pitch
    #[clap(short, long, default_value = "false")]
    pitch: bool,

    /// Set the amount of pitch deviation from 0 - 0.99
    #[clap(short = 'd', long, default_value = "0.2", value_parser = validate_pitch_deviation)]
    pitch_deviation: f32,
}

use oqs::*;
fn main() -> Result<()> {
    let cli = Cli::parse();
    let kemalg = kem::Kem::new(kem::Algorithm::Kyber1024)?;

    // A -> B: kem_pk, signature
    let (kem_pk, kem_sk) = kemalg.keypair()?;

    // B -> A: kem_ct, signature
    let (kem_ct, b_kem_ss) = kemalg.encapsulate(&kem_pk)?;

    // A verifies, decapsulates, now both have kem_ss
    let a_kem_ss = kemalg.decapsulate(&kem_sk, &kem_ct)?;
    assert_eq!(a_kem_ss, b_kem_ss);

    println!("Hei!");

    Ok(())
}
