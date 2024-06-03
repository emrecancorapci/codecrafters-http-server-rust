use std::env;

pub fn get_env_arg(argument: &str) -> Result<String, String> {
    let env_args: Vec<String> = env::args().collect();

    match
        env_args
            .iter()
            .position(|arg| arg == argument)
            .map(move |index| env_args[index + 1].to_string())
    {
        Some(env_arg) => { Ok(env_arg) }
        None => { Err(format!("Enviroment argument is unavailable: {}", argument)) }
    }
}

pub fn get_env_args(argument: &str, number_of_args: usize) -> Result<Vec<String>, String> {
    let env_args: Vec<String> = env::args().collect();

    match
        env_args
            .iter()
            .position(|arg| arg == argument)
            .map(|index|
                env_args
                    .iter()
                    .skip(index + 1)
                    .take(number_of_args)
                    .map(|arg| arg.to_string())
                    .collect()
            )
    {
        Some(env_arg) => { Ok(env_arg) }
        None => { Err(format!("Enviroment argument is unavailable: {}", argument)) }
    }
}
