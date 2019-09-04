mod lightclient;
mod lightwallet;
mod address;
mod prover;

use lightclient::LightClient;

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub mod grpc_client {
    include!(concat!(env!("OUT_DIR"), "/cash.z.wallet.sdk.rpc.rs"));
}



pub fn main() {
    let light_client = LightClient::new();

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                do_user_command(line, &light_client);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

pub fn do_user_command(cmd: String, light_client: &LightClient) {
    match cmd.as_ref() {
        "sync"    => { 
                        light_client.do_sync();
                    }
        "address" => {
                        
                    }                    
        _         => { println!("Unknown command {}", cmd); }
    }
}
