#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

//mod menu;

fn main() {
	println!("Creating app");
    tauri::Builder::default()
        // .manage(MG(Default::default()))
        // .invoke_handler(tauri::generate_handler![        ])
        .on_menu_event(|event| {
            println!("{:?}", event.menu_item_id());
        })
        //If using rust-analyzer, this line may give a false error.
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
