// Copyright 2022 The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
#[macro_use]
extern crate lazy_static;

use std::thread::{self};

use log::*;

mod api;
mod commands;
mod docker;
mod error;
mod grpc;
mod rest;

use docker::{shutdown_all_containers, DockerWrapper, Workspaces, DOCKER_INSTANCE};
use tauri::{
    api::cli::get_matches,
    async_runtime::block_on,
    utils::config::CliConfig,
    GlobalWindowEvent,
    Menu,
    MenuItem,
    PackageInfo,
    Submenu,
    WindowEvent,
};
use tauri_plugin_sql::{Migration, MigrationKind, TauriSql};

use crate::{
    api::{
        base_node_sync_progress,
        delete_seed_words,
        get_seed_words,
        health_check,
        image_info,
        network_list,
        node_identity,
        transaction_fee,
        transfer,
        wallet_balance,
        wallet_events,
        wallet_identity,
    },
    commands::{
        check_docker,
        check_internet_connection,
        clean_docker,
        create_default_workspace,
        create_new_workspace,
        events,
        launch_docker,
        open_terminal,
        pull_image,
        pull_images,
        shutdown,
        start_service,
        stop_service,
        AppState,
    },
    docker::DEFAULT_WORKSPACE_NAME,
};

fn main() {
    env_logger::init();
    let context = tauri::generate_context!();
    let cli_config = context.config().tauri.cli.clone().unwrap();

    // We're going to attach this to the AppState because Tauri does not expose it for some reason
    let package_info = context.package_info().clone();
    // Handle --help and --version. Exits after printing
    handle_cli_options(&cli_config, &package_info);
    let docker = match DockerWrapper::new() {
        Ok(docker) => docker,
        Err(err) => {
            error!("Could not launch docker backend. {}", err.chained_message());
            std::process::exit(-1);
        },
    };
    thread::spawn(|| block_on(shutdown_all_containers(DEFAULT_WORKSPACE_NAME, &DOCKER_INSTANCE)));

    let menu = create_menus();
    // TODO - Load workspace definitions from persistent storage here
    let workspaces = Workspaces::default();
    info!("Using Docker version: {}", docker.version());
    let migrations = do_migrations();
    tauri::Builder::default()
        .plugin(TauriSql::default().add_migrations("sqlite:launchpad.db", migrations))
        .manage(AppState::new(docker, workspaces, package_info))
        .menu(menu)
        .invoke_handler(create_handler!())
        .on_window_event(on_event)
        .run(context)
        .expect("error starting");
    info!("At exit here!");
}

#[macro_export]
macro_rules! create_handler {
    () => {
        tauri::generate_handler![
            base_node_sync_progress,
            create_new_workspace,
            create_default_workspace,
            delete_seed_words,
            events,
            get_seed_words,
            health_check,
            image_info,
            network_list,
            pull_image,
            pull_images,
            check_docker,
            launch_docker,
            check_internet_connection,
            open_terminal,
            node_identity,
            clean_docker,
            start_service,
            stop_service,
            shutdown,
            transaction_fee,
            transfer,
            wallet_events,
            wallet_balance,
            wallet_identity,
        ]
    };
}

fn do_migrations() -> Vec<Migration> {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create stats table",
            sql: include_str!("../migrations/2022-06-13.create-stats-table.sql"),
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "create transactions table",
            sql: include_str!("../migrations/2022-06-14.create-transactions-table.sql"),
            kind: MigrationKind::Up,
        },
    ];
    migrations
}

fn create_menus() -> Menu {
    let about_menu = Submenu::new(
        "App",
        Menu::new()
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::HideOthers)
            .add_native_item(MenuItem::ShowAll)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Quit),
    );

    let edit_menu = Submenu::new(
        "Edit",
        Menu::new()
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::SelectAll),
    );

    let view_menu = Submenu::new("View", Menu::new().add_native_item(MenuItem::EnterFullScreen));

    Menu::new()
        .add_submenu(about_menu)
        .add_submenu(edit_menu)
        .add_submenu(view_menu)
}

fn on_event(evt: GlobalWindowEvent) {
    if let WindowEvent::Destroyed = evt.event() {
        info!("Stopping and destroying all tari containers");
        let task = thread::spawn(|| {
            block_on(shutdown_all_containers(
                DEFAULT_WORKSPACE_NAME,
                &DOCKER_INSTANCE.clone(),
            ))
        });
        let _unused = task.join();
    }
}

fn handle_cli_options(cli_config: &CliConfig, pkg_info: &PackageInfo) {
    match get_matches(cli_config, pkg_info) {
        Ok(matches) => {
            if let Some(arg_data) = matches.args.get("help") {
                debug!("{}", arg_data.value.as_str().unwrap_or("No help available"));
                std::process::exit(0);
            }
            if let Some(arg_data) = matches.args.get("version") {
                debug!("{}", arg_data.value.as_str().unwrap_or("No version data available"));
                std::process::exit(0);
            }
        },
        Err(e) => {
            error!("{}", e.to_string());
            std::process::exit(1);
        },
    }
}
