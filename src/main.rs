use std::error::Error;
use std::time::Duration;
use std::{env, thread};

// i wonder how can I write more reusable code here...
fn usage_error_handler() {
    let usage_text = format!("{} 2 http://example.com/", env::args().nth(0).unwrap());
    println!("{}", &usage_text);
    std::process::exit(1);
}

fn interval_error_handler() {
    let invalid_interval_text = "Interval should be positive integer number and be greater than 0.";
    println!("{}", &invalid_interval_text);
    std::process::exit(2);
}

fn could_not_connect_handler() {
    let could_not_connect_text = "Could not establish connection with the specified endpoint.";
    println!("{}", &could_not_connect_text);
    std::process::exit(3);
}

fn could_not_get_status_code_handler() {
    let could_not_get_status_code = "Program couldnt get the status code from the response.";
    println!("{}", &could_not_get_status_code);
    std::process::exit(3);
}

fn get_from_option_or_exit<R, F: Fn() -> ()>(option: Option<R>, closure: F) -> R {
    match option {
        Some(arg) => arg,
        None => {
            closure();
            panic!("");
        }
    }
}

fn get_from_result_or_exit<R, E, F: Fn() -> ()>(result: Result<R, E>, closure: F) -> R {
    match result {
        Ok(arg) => arg,
        Err(_) => {
            closure();
            panic!("")
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let https_string = "https://";
    let http_string = "http://";

    // get interval and endpoint arguments
    let arg_interval_string = get_from_option_or_exit(env::args().nth(1), usage_error_handler);

    let arg_endpoint_string = get_from_option_or_exit(env::args().nth(2), usage_error_handler);
    // append this string with protocol if not present
    let mut endpoint = String::new();
    if !arg_endpoint_string.starts_with(&https_string)
        || !arg_endpoint_string.starts_with(&http_string)
    {
        endpoint.push_str(&https_string);
    }
    endpoint.push_str(&*arg_endpoint_string);

    // interval should be a valid unsigned integer
    let interval =
        get_from_result_or_exit(arg_interval_string.parse::<u64>(), interval_error_handler);
    if interval == 0 {
        interval_error_handler();
    }

    // start sending requests
    loop {
        thread::sleep(Duration::from_secs(interval));
        // make the request to the specified endpoint
        let response =
            get_from_result_or_exit(reqwest::blocking::get(&endpoint), could_not_connect_handler);
        match response.error_for_status() {
            Ok(_) => {
                println!("Checking '{}'. Result: OK(200)", &endpoint);
            }
            Err(err) => {
                println!(
                    "Checking '{}'. Result: ERR({})",
                    &endpoint,
                    get_from_option_or_exit(err.status(), could_not_get_status_code_handler)
                        .as_str()
                );
            }
        }
    }
}
