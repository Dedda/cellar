use jni::sys::{jstring, jint, jobjectArray};
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::jnitools::{CLASS_NAME_STRING, jstring_to_string};

mod cell;
use cell::Cell;

mod row;

mod table;
use table::Table;
use std::iter::FromIterator;

struct Folder {
    tables: HashMap<String, Table>,
}

impl Folder {
    fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    fn get_table(&self, id: &str) -> Option<&Table> {
        self.tables.get(id)
    }

    fn get_table_mut(&mut self, id: &str) -> Option<&mut Table> {
        self.tables.get_mut(id)
    }

    fn create_table_if_not_exists(&mut self, id: &str) {
        if self.get_table(id).is_none() {
            self.tables.insert(id.into(), Table::new());
        }
    }
}

lazy_static! {
    static ref FOLDERS: Arc<Mutex<HashMap<String, Folder>>> = Arc::new(Mutex::new(HashMap::new()));
}

fn create_folder_if_not_exists(id: &str) {
    let mut folders = FOLDERS.lock().unwrap();
    if !folders.contains_key(id) {
        folders.insert(id.into(), Folder::new());
    }
}

#[no_mangle]
pub extern fn Java_org_dedda_cellar_core_GridCore_contentAt(
    env: JNIEnv,
    _class: JClass,
    folder_id: JString,
    table_id: JString,
    x: jint,
    y: jint,
) -> jstring {
    let folders = FOLDERS.lock().unwrap();
    let folder_id = jstring_to_string(&env, folder_id);
    let table_id = jstring_to_string(&env, table_id);
    let content = folders.get(&folder_id)
        .map(|f| f.get_table(&table_id)).flatten()
        .map(|g| g.get_cell(x as usize, y as usize)).flatten()
        .map(|c| c.get_content().clone()).unwrap_or(String::new());
    env.new_string(content).unwrap().into_inner()
}

#[no_mangle]
pub extern fn Java_org_dedda_cellar_core_GridCore_renderedAt(
    env: JNIEnv,
    _class: JClass,
    folder_id: JString,
    table_id: JString,
    x: jint,
    y: jint,
) -> jstring {
    let folders = FOLDERS.lock().unwrap();
    let folder_id = jstring_to_string(&env, folder_id);
    let table_id = jstring_to_string(&env, table_id);
    let content = folders.get(&folder_id)
        .map(|f| f.get_table(&table_id)).flatten()
        .map(|g| g.get_cell(x as usize, y as usize)).flatten()
        .map(|c| c.rendered.clone()).unwrap_or(String::new());
    env.new_string(content).unwrap().into_inner()
}

#[no_mangle]
pub extern fn Java_org_dedda_cellar_core_GridCore_putContentAt(
    env: JNIEnv,
    _class: JClass,
    folder_id: JString,
    table_id: JString,
    x: jint,
    y: jint,
    content: JString,
) {
    let folder_id: String = env.get_string(folder_id).unwrap().to_str().unwrap().into();
    let table_id: String = env.get_string(table_id).unwrap().to_str().unwrap().into();
    let content: String = env.get_string(content).unwrap().to_str().unwrap().into();
    create_folder_if_not_exists(&folder_id);
    let mut folders = FOLDERS.lock().unwrap();
    folders.get_mut(&folder_id).map(|f| {
        f.create_table_if_not_exists(&table_id);
        f.get_table_mut(&table_id)
    }).flatten()
        .map(|t| {
            t.generate_cells_until(x.clone() as usize, y.clone() as usize);
            t.get_cell_mut(x as usize, y as usize)
        }).flatten()
        .map(|c| c.update_content(content));
}

#[no_mangle]
pub extern fn Java_org_dedda_cellar_core_GridCore_tableIdsForFolderId(
    env: JNIEnv,
    _class: JClass,
    folder_id: JString,
) -> jobjectArray {
    let folders = FOLDERS.lock().unwrap();
    if let Some(folder) = folders.get(env.get_string(folder_id).unwrap().to_str().unwrap()) {
        let keys = Vec::from_iter(folder.tables.keys().cloned());
        let arr = env.new_object_array(
            keys.len() as i32,
            env.find_class(CLASS_NAME_STRING).unwrap(),
            env.new_string("").unwrap()).unwrap();
        keys.into_iter().enumerate().for_each(|(index, name)| {
            env.set_object_array_element(arr, index as i32, env.new_string(name).unwrap()).unwrap();
        });
        arr
    } else {
        env.new_object_array(0, env.find_class(CLASS_NAME_STRING).unwrap(), env.new_string("").unwrap()).unwrap()
    }
}
