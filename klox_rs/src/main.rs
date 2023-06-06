use klox_rs::vm::{InterpreterError, VM};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
fn main() -> Result<(), InterpreterError> {
    if std::env::args().len() == 1 {
        repl()?;
    } else if std::env::args().len() == 2 {
        run_file(std::env::args().nth(1).unwrap())?;
    } else {
        println!("Usage: klox [path]");
    }

    Ok(())
}

fn repl() -> Result<(), InterpreterError> {
    let mut vm = VM::new();
    let mut rl = DefaultEditor::new().map_err(|_| InterpreterError::SourceError)?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline("Lox> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())
                    .map_err(|_| InterpreterError::SourceError)?;
                vm.interpret(line)?;
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
    Ok(())
}

fn run_file(path: String) -> Result<(), InterpreterError> {
    let mut vm = VM::new();
    let source = std::fs::read_to_string(path).map_err(|_| InterpreterError::SourceError)?;
    vm.interpret(source)?;
    Ok(())
}
