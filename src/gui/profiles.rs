use gtk::gio::File;
use gtk::{prelude::*, Button, DropDown, FileDialog, Grid, ScrolledWindow};
use serde_json;
use std::fs;
use std::sync::{Arc, Mutex};

use crate::save::AppPaths;
use crate::structs::Controller;

fn apply_profile_from_path(path: &std::path::Path, controller: &Arc<Mutex<Controller>>) {
    if let Ok(contents) = fs::read_to_string(path) {
        match serde_json::from_str::<Controller>(&contents) {
            Ok(profile) => {
                let mut controller_lock = controller.lock().unwrap();
                *controller_lock = profile;
                println!("Controller state updated from imported profile.");
            }
            Err(err) => eprintln!("Failed to parse imported profile: {}", err),
        }
    } else {
        eprintln!("Failed to read profile file.");
    }
}

pub fn create_profiles_page(
    controller: &Arc<Mutex<Controller>>,
    app_paths: &Arc<AppPaths>,
) -> ScrolledWindow {
    let controller = Arc::clone(controller);
    let app_paths = Arc::clone(app_paths);

    let presets_grid = Grid::builder()
        .row_spacing(10)
        .column_spacing(10)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let export_button = Button::with_label("Export Profile");
    let import_button = Button::with_label("Import Profile");
    let dropdown = DropDown::builder()
        .model(&gtk::StringList::new(&["Profile1", "Profile2", "Profile3"]))
        .selected(0)
        .build();

    let dropdown_clone = dropdown.clone();

    let refresh_dropdown = {
        let app_paths = Arc::clone(&app_paths);
        move || {
            let profiles_dir = &app_paths.profiles;
            if let Ok(entries) = fs::read_dir(profiles_dir) {
                let profiles: Vec<String> = entries
                    .filter_map(|entry| entry.ok())
                    .filter_map(|entry| entry.path().file_stem()?.to_str().map(|s| s.to_string()))
                    .collect();

                dropdown_clone.set_model(Some(&gtk::StringList::new(
                    &profiles.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
                )));
            }
        }
    };

    refresh_dropdown();

    {
        let controller_export = Arc::clone(&controller);
        let app_paths_export = Arc::clone(&app_paths);

        export_button.connect_clicked(move |_| {
            let dialog = FileDialog::new();
            dialog.set_modal(true);
            dialog.set_initial_folder(Some(&File::for_path(&app_paths_export.profiles)));

            let controller_for_open = Arc::clone(&controller_export);
            dialog.save(
                None::<&gtk::Window>,
                Option::<&gtk::gio::Cancellable>::None,
                move |result| match result {
                    Ok(file) => {
                        if let Some(path) = file.path() {
                            let controller_lock = controller_for_open.lock().unwrap();
                            let json = match serde_json::to_string_pretty(&*controller_lock) {
                                Ok(json) => json,
                                Err(err) => {
                                    eprintln!("Failed to serialize Controller: {}", err);
                                    return;
                                }
                            };

                            match fs::write(&path, json) {
                                Ok(_) => println!("Profile exported to {}", path.display()),
                                Err(err) => eprintln!("Failed to export profile: {}", err),
                            }
                        } else {
                            eprintln!("No valid path provided.");
                        }
                    }
                    Err(err) => eprintln!("Error exporting profile: {}", err),
                },
            );
        });
    }

    {
        let app_paths_import = Arc::clone(&app_paths);
        let controller_import = Arc::clone(&controller);
        let refresh_dropdown_import = refresh_dropdown.clone();

        import_button.connect_clicked(move |_| {
            let dialog = FileDialog::new();
            dialog.set_modal(true);
            dialog.set_initial_folder(Some(&File::for_path(&app_paths_import.profiles)));

            let controller_for_open = Arc::clone(&controller_import);
            let app_paths_for_open = Arc::clone(&app_paths_import);
            let refresh_dropdown_for_open = refresh_dropdown_import.clone();

            dialog.open(
                None::<&gtk::Window>,
                Option::<&gtk::gio::Cancellable>::None,
                move |result| match result {
                    Ok(file) => {
                        if let Some(path) = file.path() {
                            if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                                let target_path = app_paths_for_open.profiles.join(
                                    path.file_name()
                                        .unwrap_or_else(|| std::ffi::OsStr::new("profile.json")),
                                );

                                // Check if file is already in the profiles directory
                                if path
                                    .parent()
                                    .map_or(false, |p| p == app_paths_for_open.profiles)
                                {
                                    apply_profile_from_path(&path, &controller_for_open);
                                    refresh_dropdown_for_open();
                                } else {
                                    match fs::copy(&path, &target_path) {
                                        Ok(_) => {
                                            apply_profile_from_path(
                                                &target_path,
                                                &controller_for_open,
                                            );
                                            refresh_dropdown_for_open();
                                        }
                                        Err(err) => eprintln!("Failed to import profile: {}", err),
                                    }
                                }
                            } else {
                                eprintln!("Invalid file type. Please select a .json file.");
                            }
                        } else {
                            eprintln!("No valid path provided.");
                        }
                    }
                    Err(err) => eprintln!("Error importing profile: {}", err),
                },
            );
        });
    }

    {
        let app_paths_apply = Arc::clone(&app_paths);
        let controller_apply = Arc::clone(&controller);

        dropdown.connect_selected_notify(move |dropdown| {
            if let Some(model) = dropdown.model() {
                let selected_idx = dropdown.selected();
                if let Some(selected_name) = model
                    .downcast_ref::<gtk::StringList>()
                    .and_then(|string_list| string_list.string(selected_idx))
                {
                    let profile_path = app_paths_apply
                        .profiles
                        .join(format!("{}.json", selected_name));
                    if let Ok(contents) = fs::read_to_string(profile_path) {
                        match serde_json::from_str::<Controller>(&contents) {
                            Ok(profile) => {
                                let mut controller_lock = controller_apply.lock().unwrap();
                                *controller_lock = profile;
                                println!("Profile '{}' applied.", selected_name);
                            }
                            Err(err) => eprintln!("Failed to load profile: {}", err),
                        }
                    } else {
                        eprintln!("Failed to read profile file.");
                    }
                }
            }
        });
    }

    presets_grid.attach(&export_button, 0, 0, 1, 1);
    presets_grid.attach(&import_button, 1, 0, 1, 1);
    presets_grid.attach(&dropdown, 0, 1, 2, 1);

    ScrolledWindow::builder()
        .min_content_width(400)
        .min_content_height(400)
        .child(&presets_grid)
        .build()
}
