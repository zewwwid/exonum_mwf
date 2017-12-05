extern crate exonum;
extern crate cryptocurrency;
extern crate exonum_configuration;

use exonum::helpers;
use exonum::helpers::fabric::NodeBuilder;
use exonum_configuration::ConfigurationService;
use cryptocurrency::CurrencyService;

fn main() {
    exonum::crypto::init();
    helpers::init_logger().unwrap();

    let node = NodeBuilder::new()
        .with_service(Box::new(ConfigurationService::new()))
        .with_service(Box::new(CurrencyService::new()));
    node.run();
}
