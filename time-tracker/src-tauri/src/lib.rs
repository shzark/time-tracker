// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/


use serde::{ Deserialize,Serialize};
use sysinfo::{Process, ProcessStatus, System};


#[cfg(target_os = "macos")]
const APPLICATION_DIRS : &[&str] = &["/Applications","/Users/*/Applications"];

#[cfg(target_os = "linux")]
const APPLICATION_DIRS : &[&str] = &["/usr/bin","/usr/local/bin","/opt/bin"];

#[cfg(target_os = "windows")]
const APPLICATION_DIRS : &[&str] = &["C:\\Program Files","C:\\Program Files (x86)"];

#[derive(Serialize,Deserialize)]
struct AppInfo {
    id: String,
    name: String,
    running_time_formatted: String, // 运行时长
    memory_formatted: String
    //memory_info_bytes: u64   // 消耗的内存
}

fn is_valid(process: &Process) -> bool {
    let helper_keywords = vec!["helper","service","daemon","agent","."];

    if let Some(exe_path) = process.exe().unwrap().to_str() {
        let is_in_app_dir = APPLICATION_DIRS.iter()
            .any(|dir| exe_path.starts_with(dir));

        let is_helper = helper_keywords.iter()
            .any(|keyword| process.name().to_string_lossy().to_ascii_lowercase().contains(keyword));


        process.status() == ProcessStatus::Run && is_in_app_dir && !is_helper

    }else {
        false
    }
}

// Format the output run time
fn format_running_time(seconds : u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    format!("[ {} Days {} Hours {} Minutes {} Seconds ]",days,hours,minutes,seconds)
}

// Format the output memory size
fn format_memory_info(bytes : u64) -> String {
    let b = bytes / 1024 / 1024 ;

    format!("{} MB",b)
}


#[tauri::command]
fn max_running_process() -> Option<AppInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    sys.processes().
        iter()
        .filter(|(_,process)| is_valid(process) )
        .max_by_key(|(_, process)| process.run_time())
        .map(|(id,process)| {


            // println!("Running Time ID: {}, Name: {}, Time: {}", id.to_string(),
            //     process.name().to_string_lossy().into_owned(),
            //     format_running_time(process.run_time())
            //     );

            AppInfo {
                id: id.to_string(),
                name: process.name().to_string_lossy().into_owned(),
                running_time_formatted: format_running_time(process.run_time()),
                memory_formatted: format_memory_info(process.memory())
            }
        })
}



#[tauri::command]
fn max_memory() -> Option<AppInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    sys.processes().
        iter()
        .filter(|(_,process)| is_valid(process) )
        .max_by_key(|(_, process)| process.memory())
        .map(|(id,process)| {

            AppInfo {
                id: id.to_string(),
                name: process.name().to_string_lossy().into_owned(),
                running_time_formatted: format_running_time(process.run_time()),
                memory_formatted: format_memory_info(process.memory())
            }
        })
}


#[tauri::command]
fn list_process() -> Vec<AppInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut processes: Vec<AppInfo> = sys.processes()
        .iter()
        .filter(|(_,process)| is_valid(process) )
        .map(|(id,process) | {
            AppInfo {
                id: id.to_string(),
                name: process.name().to_string_lossy().into_owned(),
                running_time_formatted: format_running_time(process.run_time()),
                memory_formatted: format_memory_info(process.memory())
            }
        })
        .collect();

    processes.sort_by(|a,b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    processes
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![list_process,max_memory,max_running_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
