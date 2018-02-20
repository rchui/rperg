use std::env;

pub fn get_cwd() -> String {
    let current_dir = env::current_dir();
    let result = match current_dir {
        Ok(result) => result,
        Err(result) => panic!("{:?}", result)
    };
    result.into_os_string().into_string().unwrap()
}