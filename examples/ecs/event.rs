use bevy::prelude::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_event::<MyEvent>()
        .init_resource::<EventTriggerState>()
        .init_resource::<EventListenerState>()
        .add_system(event_trigger_system.system())
        .add_system(event_listener_system.system())
        .run();
}

struct MyEvent {
    pub message: String,
}

#[derive(Default)]
struct EventTriggerState {
    elapsed: f32,
}

// sends MyEvent every second
fn event_trigger_system(
    mut state: ResMut<EventTriggerState>,
    mut my_events: ResMut<Events<MyEvent>>,
    time: Res<Time>,
) {
    state.elapsed += time.delta_seconds;
    if state.elapsed > 1.0 {
        my_events.send(MyEvent {
            message: "MyEvent just happened!".to_string(),
        });

        state.elapsed = 0.0;
    }
}

#[derive(Resource)]
struct EventListenerState {
    my_event_reader: EventReader<MyEvent>,
}

// prints events as they come in
fn event_listener_system(
    mut state: ResMut<EventListenerState>,
    my_events: Res<Events<MyEvent>>,
) {
    for my_event in state.my_event_reader.iter(&my_events) {
        println!("{}", my_event.message);
    }
}
