use bend::diagnostics::{Diagnostics, DiagnosticsConfig};
use bend::fun::{Book, Term};
use bend::{CompileOpts, RunOpts};

pub fn run(book: &Book) -> Option<(Term, String, Diagnostics)> {
    let run_opts = RunOpts {
        linear_readback: false,
        pretty: false,
    };
    let compile_opts = CompileOpts::default();
    let diagnostics_cfg = DiagnosticsConfig::default();
    let args = None;

    bend::run_book(
        book.clone(),
        run_opts,
        compile_opts,
        diagnostics_cfg,
        args,
        "run",
    )
    .unwrap()
}
