use dprint_core::plugins::process::{handle_process_stdio_messages, start_parent_process_checker_thread};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// init
    #[structopt(long)]
    init: bool,
    /// parent-pid
    #[structopt(long = "parent-pid")]
    parent_pid: u32,
}

fn main() -> Result<(), dprint_core::types::ErrBox> {
    let opt = Opt::from_args();

    let parent_process_id = opt.parent_pid;
    start_parent_process_checker_thread(String::from(env!("CARGO_PKG_NAME")), parent_process_id);

    handle_process_stdio_messages(dprint_plugin_taplo::TaploPluginHandler::default())
}
