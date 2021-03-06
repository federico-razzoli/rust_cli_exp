use stylesheet::Stylesheet;


#[derive(Debug, Clone, Copy)]
pub struct Event {
    pub level: &'static str,
    pub message: &'static str,
}

impl Event {
    pub fn print(&self, sheet: &Stylesheet) {
        sheet.println(self.level, self.message);
    }
}


pub mod long_range_scanner {
    use super::Event;

    extern crate rand;
    use self::rand::Rng;


    pub fn scan() -> Event {
        // Detect a random event
        let possible_events: [Event; 4] = [
            Event { level: "info", message: "Whormhole detected" },
            Event { level: "info", message: "Vulcan ship deteced" },
            Event { level: "danger", message: "Romulan ship approaching!" },
            Event { level: "danger", message: "Borg cube approaching!" },
        ];

        let rnd = rand::thread_rng().gen_range(0, possible_events.len());
        possible_events[rnd]
    }
}
