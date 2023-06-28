use indiemotion_api::prelude::*;

#[swift_bridge::bridge]
mod ffi {

    extern "Rust" {
        type Vec3;
        type Vec4;
        type Matrix44;
        type Name;
    }

    extern "Rust" {
        type Value;

        // fn as_f64(&self) -> Option<&f64>;
        // fn as_vec3(&self) -> Option<&Vec3>;

    }

    extern "Rust" {
        type PropertyDef;

        #[swift_bridge(associated_to = PropertyDef, args_into = (name))]
        fn new(name: String, default_value: Value) -> PropertyDef;
        fn name(&self) -> &Name;
        fn default_value(&self) -> &Value;
    }

    extern "Rust" {
        type ControllerDef;
        #[swift_bridge(associated_to = ControllerDef, args_into = (name))]
        fn new(name: String, properties: Vec<PropertyDef>) -> ControllerDef;

        fn name(&self) -> &Name;
        // fn properties(&self) -> &Vec<PropertyDef>;
        fn property(&self, name: &Name) -> Option<&PropertyDef>;
    }
}
