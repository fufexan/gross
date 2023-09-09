use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use hyprland::{
    data::{Monitors, Workspace, Workspaces},
    event_listener,
    shared::{HyprData, HyprError, WorkspaceType},
};
use serde_json::json;

#[derive(Debug, serde::Serialize, Clone)]
enum WorkspaceState {
    Empty,
    Active,
}

#[derive(Debug, serde::Serialize, Clone)]
struct Ws {
    id: i32,
    name: String,
    state: WorkspaceState,
    monitor: i16,
}

impl From<Workspace> for Ws {
    fn from(value: Workspace) -> Self {
        let monitors = mon_from_monitors(Monitors::get());
        Self {
            id: value.id,
            name: value.name,
            state: WorkspaceState::Active,
            monitor: *monitors.get(&value.monitor).unwrap_or(&0),
        }
    }
}

#[derive(Debug, serde::Serialize, Clone)]
struct Hyprland {
    focused: WorkspaceType,
    workspaces: Vec<Ws>,
    screenshare: bool,
}

#[tokio::main]
pub async fn main() {
    let mut listener = event_listener::EventListener::new();

    // set initial values
    let workspaces = ws_from_workspaces(Workspaces::get());
    let hyprland = Arc::new(Mutex::new(Hyprland {
        focused: WorkspaceType::Regular(String::from("1")),
        workspaces,
        screenshare: false,
    }));

    // print initial values
    println!("{}", json!(*hyprland));

    // handle workspace changes
    let hl = Arc::clone(&hyprland);
    listener.add_workspace_change_handler(move |id| {
        hl.lock().unwrap().focused = id;

        println!("{}", json!(*hl));
    });

    let hl = Arc::clone(&hyprland);
    listener.add_active_monitor_change_handler(move |event| {
        hl.lock().unwrap().focused = event.workspace;

        println!("{}", json!(*hl));
    });

    // handle workspace add/remove
    let hl = Arc::clone(&hyprland);
    let handle_add_remove = move |_| {
        hl.lock().unwrap().workspaces = ws_from_workspaces(Workspaces::get());

        println!("{}", json!(*hl));
    };

    listener.add_workspace_added_handler(handle_add_remove.clone());
    listener.add_workspace_destroy_handler(handle_add_remove);

    // handle screenshare
    let hl = Arc::clone(&hyprland);
    listener.add_screencast_handler(move |event| {
        hl.lock().unwrap().screenshare = event.is_turning_on;

        println!("{}", json!(*hl));
    });

    // start event listener
    listener
        .start_listener_async()
        .await
        .expect("Could not start event listener");
}

fn mon_from_monitors(monitors: Result<Monitors, HyprError>) -> HashMap<String, i16> {
    monitors.map_or_else(
        |_| {
            vec![1]
                .iter()
                .map(|_| (String::from("eDP-1"), 0))
                .collect::<HashMap<_, _>>()
        },
        |monitors| monitors.map(|m| (m.name, m.id)).collect::<HashMap<_, _>>(),
    )
}

fn ws_from_workspaces(workspaces: Result<Workspaces, HyprError>) -> Vec<Ws> {
    // create empty Ws based on id
    let empty = move |id| Ws {
        id,
        name: id.to_string(),
        state: WorkspaceState::Empty,
        monitor: 0,
    };

    // create vec of ws from Workspaces iter, or one-ws vec if it fails
    let mut wss: Vec<Ws> =
        workspaces.map_or_else(|_| vec![empty(1)], |ws| ws.map(Ws::from).collect());
    let last = wss.iter().map(|w| w.id).max().unwrap_or_default();

    let orig_len = wss.len();

    // fill any workspaces between 1 and n
    for i in 1..=last {
        if !wss[..orig_len].iter().any(|e| e.id == i) {
            wss.push(empty(i));
        }
    }

    // sort
    wss.sort_by_key(|w| w.id);

    // create n+1 workspace
    if last < 10 {
        let id = last + 1;
        wss.push(empty(id));
    }

    wss
}
