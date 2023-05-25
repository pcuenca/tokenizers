extern crate tokenizers as tk;

use crate::trainers::Trainer;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;
use std::path::PathBuf;
use std::ptr;
use std::sync::{Arc, RwLock};

use tk::models::{
    bpe::{BpeBuilder, Merges, Vocab},
    wordlevel::{WordLevel, WordLevelBuilder},
    wordpiece::WordPieceBuilder,
    ModelWrapper,
};

use tk::Model as ModelTrait;
use tk::Token;

/// Model
#[derive(Clone, Serialize, Deserialize)]
pub struct Model {
    #[serde(flatten)]
    pub model: Option<Arc<RwLock<ModelWrapper>>>,
}


impl<M> From<M> for Model
where
    M: Into<ModelWrapper>,
{
    fn from(wrapper: M) -> Self {
        Self {
            model: Some(Arc::new(RwLock::new(wrapper.into()))),
        }
    }
}

impl tk::Model for Model {
    type Trainer = Trainer;

    fn tokenize(&self, sequence: &str) -> tk::Result<Vec<Token>> {
        self.model
            .as_ref()
            .ok_or("Uninitialized Model")?
            .read()
            .unwrap()
            .tokenize(sequence)
    }

    fn token_to_id(&self, token: &str) -> Option<u32> {
        self.model.as_ref()?.read().unwrap().token_to_id(token)
    }

    fn id_to_token(&self, id: u32) -> Option<String> {
        self.model.as_ref()?.read().unwrap().id_to_token(id)
    }

    fn get_vocab(&self) -> HashMap<String, u32> {
        self.model
            .as_ref()
            .expect("Uninitialized Model")
            .read()
            .unwrap()
            .get_vocab()
    }

    fn get_vocab_size(&self) -> usize {
        self.model
            .as_ref()
            .expect("Uninitialized Model")
            .read()
            .unwrap()
            .get_vocab_size()
    }

    fn save(&self, folder: &Path, name: Option<&str>) -> tk::Result<Vec<PathBuf>> {
        self.model
            .as_ref()
            .ok_or("Uninitialized Model")?
            .read()
            .unwrap()
            .save(folder, name)
    }

    fn get_trainer(&self) -> Self::Trainer {
        self.model
            .as_ref()
            .expect("Uninitialized Model")
            .read()
            .unwrap()
            .get_trainer()
            .into()
    }
}

// Type conversions

pub unsafe fn c_to_rust_string(c_string: *const c_char) -> Option<String> {
    if c_string.is_null() {
        None
    } else {
        // Note: the C string must live while we use the String
        // TODO: better error handling
        let c_str = CStr::from_ptr(c_string);
        let r_str = c_str.to_str().expect("Failed to convert C string to Rust string");
        Some(r_str.to_owned())
    }
}

pub fn rust_string_to_c(r_string: Option<String>) -> *const c_char {
    match r_string {
        Some(s) => {
            let c_string = CString::new(s).expect("Failed to convert Rust string to C string");
            let c_ptr = c_string.as_ptr();
            // This memory will not be freed! The C code should call some Rust function to free it :(
            std::mem::forget(c_string);
            c_ptr
        }
        None => std::ptr::null(),
    }
}

// Free String previously allocated in Rust and returned to C as a c_char
#[no_mangle]
pub extern "C" fn string_destroy(c_string: *mut c_char) {
    if !c_string.is_null() {
        unsafe {
            drop(CString::from_raw(c_string));
        }
    }
}

// Starting with WordLevel as it seems easier.

// NOTE: `vocab` as a dictionary is not supported (no direct translation to C).

#[no_mangle]
pub extern "C" fn wordlevel_create(vocab_filename: *const c_char, unk_token: *const c_char) -> *mut Model {
    // Return NULL if no vocab_filename is provided
    let vocab_filename = match unsafe { c_to_rust_string(vocab_filename) } {
        Some(s) => s,
        None => return ptr::null_mut(),
    };

    let mut builder = WordLevel::builder();
    builder = builder.files(vocab_filename);

    if let Some(unk_token) = unsafe { c_to_rust_string(unk_token) } {
        builder = builder.unk_token(unk_token);
    }

    match builder.build() {
        Ok(model) => Box::into_raw(Box::new(model.into())),
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn wordlevel_destroy(model: *mut Model) {
    if !model.is_null() {
        unsafe { Box::from_raw(model) };
    }
}


// Return u32::MAX if the token is unknown.
// Not fond of it, but also not fond of returning an error code and accepting a pointer for the result.
#[no_mangle]
pub extern "C" fn token_to_id(model: *mut Model, token: *const c_char) -> u32 {
    assert!(!model.is_null());
    let model = unsafe { &*model };

    let token = match unsafe { c_to_rust_string(token) } {
        Some(s) => s,
        None => return u32::MAX,
    };

    // LOL: no semicolon returns the value (sorry, I'm new to Rust)
    model.token_to_id(&token[..]).unwrap_or(u32::MAX)
}
