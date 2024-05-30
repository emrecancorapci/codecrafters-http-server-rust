use std::env;

pub fn get_env_arg(argument: &str) -> String {
    let env_args: Vec<String> = env::args().collect();

    env_args
        .iter()
        .position(|arg| arg == argument)
        .map(|index| env_args[index + 1].clone())
        .unwrap_or(String::new())
}

pub fn get_env_args(argument: &str, num: usize) -> Vec<String> {
    let env_args: Vec<String> = env::args().collect();

    env_args
        .iter()
        .position(|arg| arg == argument)
        .map(|index|
            env_args
                .iter()
                .skip(index + 1)
                .take(num)
                .map(|arg| arg.clone())
                .collect()
        )
        .unwrap_or(Vec::new())
}
