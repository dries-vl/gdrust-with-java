use std::ffi::{c_char, CStr};

use godot::prelude::*;

use crate::nativelib;
use crate::nativelib::{graal_attach_thread, graal_create_isolate, graal_create_isolate_params_t, graal_get_current_thread, graal_isolate_t, graal_isolatethread_t};

/// Registers the class in Godot; requires engine restart
/// Inherits Sprite2D; if omitted it inherits RefCounted
#[derive(GodotClass)]
#[class(base = Node)]
struct GameData {
    pub graal_thread: *mut graal_isolatethread_t,
    /// Needed to access parent class instance because rust has no inheritance
    /// Can omit if no need for base class within self
    #[base]
    node: Base<Node>,
}

#[godot_api]
impl GameData {
    #[func]
    pub fn load_countries(&self) -> () {
        unsafe {
            nativelib::load_countries(self.graal_thread);
        }
    }
    #[func]
    pub fn get_country(&self, index: i32) -> Gd<Country> {
        unsafe {
            let country_ptr = nativelib::get_country(self.graal_thread, index) as *const Country;
            // need a copy, because Godot sometimes moves the struct elsewhere and frees the original memory
            let copied_country = *country_ptr; // assigning always copies if Copy is implemented
            let copied_country_ptr = &copied_country as *const Country;
            if (country_ptr == copied_country_ptr) {
                panic!("!!! DID NOT CREATE COPY OF COUNTRY !!!")
            }
            // !!! fails and crashes the (c++, cannot catch) application if already exists !!!
            Gd::new(Country {id: copied_country.id, name: copied_country.name})
        }
    }
    #[func]
    pub fn update_country(&self, index: i32) -> () {
        unsafe {
            nativelib::update_country(self.graal_thread, index);
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[derive(GodotClass)]
struct Country {
    id: i64, // 64 bit pointer to the c char array
    name: i64, // 64 bit pointer to the c char array
}

#[godot_api]
impl Country {
    #[func]
    pub fn get_id(&self) -> GodotString {
        /// Makes a copy of the string data and returns it as a godot string
        let c_string = unsafe { CStr::from_ptr(self.id as *const c_char) };
        let rust_string = c_string.to_str().map(|s| s.to_owned()).unwrap_or(String::from("NOT FOUND IN C"));
        let godot_string = GodotString::from(rust_string);
        return godot_string;
    }
    #[func]
    pub fn get_name(&self) -> GodotString {
        /// Makes a copy of the string data and returns it as a godot string
        let c_string = unsafe { CStr::from_ptr(self.name as *const c_char) };
        let rust_string = c_string.to_str().map(|s| s.to_owned()).unwrap_or(String::from("NOT FOUND IN C"));
        return GodotString::from(rust_string);
    }
}

/// Needed to expose this also to gdextension;
/// Every godot engine class has a {ClassName} Virtual trait
#[godot_api]
impl NodeVirtual for GameData {
    fn init(node: Base<Node>) -> Self {
        let mut params: graal_create_isolate_params_t = unsafe { std::mem::zeroed() };
        let mut isolate: *mut graal_isolate_t = std::ptr::null_mut();
        let mut thread: *mut graal_isolatethread_t = std::ptr::null_mut();
        unsafe { graal_create_isolate(&mut params, &mut isolate, &mut thread) };
        unsafe { graal_attach_thread(isolate, &mut thread) };
        let graal_thread = unsafe { graal_get_current_thread(isolate) };

        Self {
            graal_thread,
            node,
        }
    }
}
