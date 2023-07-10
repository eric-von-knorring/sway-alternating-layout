use std::process;

use swayipc::{Connection, Event, EventStream, EventType, Fallible, Node, NodeLayout, WindowChange};

pub fn start_alternating_layout() {
    let (subscription, mut command) = connect();

    for event in subscription {
        handle_event(event, &mut command);
    };
}

fn connect() -> (EventStream, Connection) {
    let subscription = match Connection::new()
        .and_then(|connection| connection.subscribe([EventType::Window])) {
        Ok(conn) => conn,
        Err(error) => {
            eprintln!("Error: failed to set upp ipc subscription. {}", error);
            process::exit(exitcode::IOERR);
        }
    };

    let commands = match Connection::new() {
        Ok(conn) => conn,
        Err(error) => {
            eprintln!("Error: failed to set upp ipc connection. {}", error);
            process::exit(exitcode::IOERR);
        }
    };

    (subscription, commands)
}

fn handle_event(event: Fallible<Event>, command: &mut Connection) {
    let Ok(Event::Window(window)) = event else {
        return;
    };
    if WindowChange::Focus == window.change {
        set_layout(window.container, command);
    };
}

fn set_layout(focused: Node, command: &mut Connection) {
    if let Ok(Some(parent)) = command.get_tree().map(|tree| find_parent(&focused, tree)) {
        if NodeLayout::Tabbed == parent.layout || NodeLayout::Stacked == parent.layout {
            return;
        }
    } else {
        eprintln!("Warning: Could not find parent.");
    }

    let Ok(Some(focused_current)) = command.get_tree().map(|tree| tree.find(|node| node.id == focused.id)) else {
        eprintln!("Warning: Could not find refresh focused node.");
        return;
    };
    if focused_current.rect.height > focused_current.rect.width {
        if let Err(error) = command.run_command(format!("[con_id={}] splitv", focused_current.id)) {
            eprintln!("Error: Failed to run splitv. {}", error)
        };
    } else {
        if let Err(error) = command.run_command(format!("[con_id={}] splith", focused_current.id)) {
            eprintln!("Error: Failed to run splith. {}", error)
        };
    }
}

fn find_parent(focused: &Node, tree: Node) -> Option<Node> {
    tree.find(|node| node.nodes.iter().any(|child| child.id == focused.id))
}
