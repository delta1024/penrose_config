#[macro_export]
macro_rules! run_external_drop {
    ( $( $x:tt ),+ ) => {
	$(
            let mut command_and_args = $x.split_whitespace();
		use std::process::Command;
	    if command_and_args.clone().count() > 1 {
	    let command = command_and_args.next().unwrap();
            let args: Vec<String> = command_and_args.map(|x| x.to_string()).collect();
		let run = Command::new(command)
		    .args(&args)
		    .spawn()
		    .expect("Failed to execute process");
		drop(run);
	    } else {
		let command = command_and_args.next().unwrap();
		let run = Command::new(command)
		    .spawn()
		    .expect("Failed to execute process");
		drop(run);
	    }
	)+
    };
}
