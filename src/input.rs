use std::{
    collections::{ HashMap, HashSet }
};

use crossterm::{
    Result,
    event::{ Event, KeyCode, KeyEvent, EventStream }
};

use tokio::{
    sync::mpsc,
    stream::StreamExt,
};

#[derive(Copy, Clone, Debug)]
pub enum Cmd {
    Quit,
    DirUp,
    DirDown,
    DirOut,
    DirIn
}

#[derive(Eq, Hash)]
pub struct Chord(pub char, pub char);

impl PartialEq for Chord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

pub async fn run(mut txr: mpsc::Sender<Cmd>, f_chords: HashMap<Chord, Cmd>) -> Result<()> {
    let mut reader = EventStream::new();

    // a hashset of potential chords the next keystroke could complete
    let mut maybe_chords: HashSet<Chord> = HashSet::new();

    loop {
        // blocks on event read
        let event = reader.next().await;

        if let Some(Ok(event)) = event {
            let mut cmd: Option<&Cmd> = None;

            // make sure that single key actions and Chords play nicely
            match event {
                Event::Key(KeyEvent { code: KeyCode::Char(key), .. }) => {
                    match key {
                        'h' => cmd = Some(&Cmd::DirOut),
                        'j' => cmd = Some(&Cmd::DirDown),
                        'k' => cmd = Some(&Cmd::DirUp),
                        'l' => cmd = Some(&Cmd::DirIn),
                        
                        // The keystroke doesn't correspond with any single key actions,
                        // proceed to try and parse some Chords.
                        _ => {
                            if maybe_chords.is_empty() {
                                for f_chord in &f_chords {
                                    // detect a key that could be the start of a few chords
                                    if key == (&f_chord.0).0 {
                                        let _chord = Chord((&f_chord.0).0, (&f_chord.0).1);
                                        maybe_chords.insert(_chord);
                                    }
                                }
                            } else {
                                // potential chords stored in last iteration
                                //
                                // say we have two commands (dd, dx).
                                // Last iteration, 'd' was detected. We're looking for 'dx'
                                // So we need to check if this most recent
                                // event is 'x'.
                                for chord in maybe_chords.drain() {
                                    if key == chord.1 {
                                        cmd = f_chords.get(&chord);
                                    }
                                }
                            }
                        }
                    }
                },
                _ => {},
            }


            if let Some(cmd) = cmd {
                txr.send(*cmd).await.unwrap()
            }
        }
    }
}
