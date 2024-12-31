use cairo_vm::air_public_input::PublicInput;
use clap::{command, Parser};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use stwo_cairo_prover::cairo_air::prove_cairo;
use stwo_cairo_prover::input::memory::{MemoryBuilder, MemoryConfig};
use stwo_cairo_prover::input::vm_import::{adapt_to_stwo_input, MemoryEntry, TraceEntry};
use stwo_cairo_prover::input::ProverInput;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;
use tracing::subscriber::set_global_default;
use tracing_subscriber::filter::EnvFilter;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[arg(long = "out_file")]
    pub out_file: PathBuf,
    #[arg(long = "private_input_file")]
    pub private_input_file: PathBuf,
    #[arg(long = "public_input_file")]
    pub public_input_file: PathBuf,
    // The following parameters are for backwards compatibility, not used by stwo
    #[arg(long = "parameter_file")]
    pub parameter_file: Option<PathBuf>,
    #[arg(long = "prover_config_file")]
    pub prover_config_file: Option<PathBuf>,
}

#[derive(Deserialize)]
pub struct AirPrivateInput {
    pub trace_path: String,
    pub memory_path: String,
}

fn init_tracing(log_level: &str) {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level));

    let subscriber_builder =
        tracing_subscriber::fmt::Subscriber::builder().with_env_filter(env_filter);

    let subscriber = subscriber_builder.with_writer(std::io::stderr).finish();
    set_global_default(subscriber).expect("Failed to set subscriber");
}

fn read_private_input<P: AsRef<Path>>(file_path: P) -> AirPrivateInput {
    let src = std::fs::read_to_string(file_path).expect("Failed to read private input");
    serde_json::from_str(&src).expect("Failed to deserialize private input")
}

fn read_trace<P: AsRef<Path>>(trace_path: P) -> Vec<TraceEntry> {
    let trace_bytes = std::fs::read(trace_path).expect("Failed to read trace");
    trace_bytes
        .chunks_exact(24)
        .map(|entry| TraceEntry {
            ap: u64::from_le_bytes(entry[..8].try_into().unwrap()),
            fp: u64::from_le_bytes(entry[8..16].try_into().unwrap()),
            pc: u64::from_le_bytes(entry[16..].try_into().unwrap()),
        })
        .collect()
}

fn read_memory<P: AsRef<Path>>(memory_path: P) -> Vec<MemoryEntry> {
    let memory_bytes = std::fs::read(memory_path).expect("Failed to read memory");
    memory_bytes
        .chunks_exact(40)
        .map(|entry| MemoryEntry {
            address: u64::from_le_bytes(entry[..8].try_into().unwrap()),
            value: entry[8..]
                .chunks_exact(4)
                .map(|limb| u32::from_le_bytes(limb.try_into().unwrap()))
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap(),
        })
        .collect()
}

fn create_prover_input(
    public_input: &PublicInput,
    trace: Vec<TraceEntry>,
    memory: Vec<MemoryEntry>,
    dev_mode: bool,
) -> ProverInput {
    let public_memory_addresses = public_input
        .public_memory
        .iter()
        .map(|s| s.address as u32)
        .collect();

    adapt_to_stwo_input(
        trace.into_iter(),
        MemoryBuilder::from_iter(MemoryConfig::default(), memory.into_iter()),
        public_memory_addresses,
        &public_input.memory_segments,
        dev_mode,
    )
    .unwrap()
}

fn main() {
    let args = Args::parse();
    init_tracing("debug");

    let private_input = read_private_input(args.private_input_file);
    let trace = read_trace(private_input.trace_path);
    let memory = read_memory(private_input.memory_path);

    let public_input_src =
        std::fs::read_to_string(args.public_input_file).expect("Failed to read public input");
    let public_input: PublicInput =
        serde_json::from_str(&public_input_src).expect("Failed to deserialize public input");

    let input = create_prover_input(&public_input, trace, memory, true);

    let proof = prove_cairo::<Blake2sMerkleChannel>(input, false, true).expect("Prover error");

    std::fs::write(
        args.out_file,
        serde_json::to_string(&proof).expect("Failed to serialize proof"),
    )
    .expect("Failed to store proof");
}
