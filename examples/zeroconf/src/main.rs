use std::any::Any;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use zeroconf::prelude::*;
use zeroconf::{MdnsService, ServiceRegistration, ServiceType, TxtRecord};

#[derive(Default)]
struct Context {}

fn main() {
    let mut service = MdnsService::new(ServiceType::new("http", "tcp").unwrap(), 8080);
    let mut txt_record = TxtRecord::new();
    let context: Arc<Mutex<Context>> = Arc::default();

    txt_record.insert("foo", "bar").unwrap();

    service.set_registered_callback(Box::new(on_service_registered));
    // service.set_context(Box::new(context));
    // service.set_txt_record(txt_record);
    service.set_name("cinemotion");

    let event_loop = service.register().unwrap();
    loop {
        event_loop.poll(Duration::from_millis(100)).unwrap();
    }
}

fn on_service_registered(result: zeroconf::Result<ServiceRegistration>, _: Option<Arc<dyn Any>>) {
    let service = result.unwrap();
    println!("Service registered: {:?}", service);
}
