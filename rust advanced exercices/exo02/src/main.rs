enum Event {
    Quit,
    Click,
    KeyPressed,
    Pause,
    Resume
}

fn log_event(e: Event) {
    match e {
        Event::Quit => println!("New event logged: Quit"),
        Event::Click => println!("New event logged: Click at (45, 540)"),
        Event::KeyPressed => println!("New event logged: Key pressed: a"),
        Event::Pause => println!("New event logged: Pause"),
        Event::Resume => println!("New event logged: Resume"),
    }
}

fn main() {
    log_event(Event::Quit);
    log_event(Event::Click);
    log_event(Event::KeyPressed);
    log_event(Event::Pause);
    log_event(Event::Resume);
}
