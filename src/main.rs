#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

use std::error::Error;
use std::env;
use lambda::error::HandlerError;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct CustomEvent {
    // environment key to check
    pub env_key: String
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct CustomOutput {
    // The value we found
    pub env_value: String
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);
    Ok(())
}

fn my_handler(event: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {

    env::var(&event.env_key)
        .map(|env_value| {
            info!("Found value of {} for {}", env_value, event.env_key);
            CustomOutput {
                env_value
            }
        })
        .map_err(|e|  {
            error!("Error occurred looking up env variable {} : {:#?}", event.env_key, e);
            c.new_error(e.description())
        })

}
