use gdnative::api::*;
use gdnative::prelude::*;

mod ffilib;

/// The BackendWrapper "class"
#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_builder)]
#[user_data(gdnative::export::user_data::MutexData<BackendWrapper>)]
pub struct BackendWrapper {
    //#[property]
    //name: String, //This was for testing, you can expose properties to godot from here
    ffi_loader: ffilib::FfiWrapper
}

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl BackendWrapper {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
        //TODO: remove this print when not needed
        //godot_print!("Game builder is registered! Rust wrapper is running...");
    }

    /// The "constructor" of the class.
    fn new(_owner: &Node) -> Self {
        //godot_print!("Game is created!");
        BackendWrapper {
            //name: "This is a property".to_string(), //This was for testing, you can expose properties to godot from here
            ffi_loader: ffilib::FfiWrapper::new(),
        }
    }


    //This method is exported and Godot engine can use it to call the GoLang backend
    #[export]
    unsafe fn go_method_call(&mut self, _owner: &Node, method_name: String, data: String) -> String {
        let res = self.ffi_loader.go_method_call(method_name, data).unwrap_or("{\"errorId\":100,\"errorDescription\":\"Error: Rust wrapper failed!\"}".to_owned());
        return res;
    }


    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are "attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    unsafe fn _ready(&mut self, _owner: &Node) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.

        //This was for testing, you can expose properties to godot from here
        //self.name = "Game".to_string();
        //godot_print!("{} is ready!", self.name);
    }

    // This function will be called in every frame
    #[export]
    unsafe fn _process(&self, _owner: &Node, delta: f64) {
        //godot_print!("Inside {} _process(), delta is {}", self.name, delta);
    }
}
