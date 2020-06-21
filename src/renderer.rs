use std::{
    env,
    collections::HashMap,
    io::Write,
};

use tokio::sync::mpsc;

use crossterm::{
    terminal, cursor,
    execute, queue, style,
    Result
};

use crate::input::{self, Cmd, Chord};

use crate::filer::Dir;

pub async fn run<W>(mut w: W) -> Result<()> where W: Write {
    // f_chords meaning "Functional Chords", chords that have a command attached to them
    let mut f_chords: HashMap<Chord, Cmd> = HashMap::new();
    f_chords.insert(Chord('q', 'q'), Cmd::Quit);
    f_chords.insert(Chord('a', 'k'), Cmd::DirUp);

    let (inp_txr, mut inp_recvr) = mpsc::channel::<Cmd>(5);
    tokio::spawn(input::run(inp_txr, f_chords));

    ////////////////////////////////////////////////////

    let cur_dir = Dir::new(env::current_dir()?).await; 
    
    // current Dir exists
    if let Ok(mut cur_dir) = cur_dir {
        init_buffer(&mut w)?;
        loop {
            /*** recv input messages ***/
            match inp_recvr.try_recv() {
                Ok(cmd) => {
                    match cmd {
                        Cmd::Quit => break,
                        Cmd::DirUp => { // move selection in child dir
                            if let Some(_) = &cur_dir.child_entries {
                                if cur_dir.selected_entry_index > 0 {
                                    cur_dir.selected_entry_index -= 1;
                                }
                            }
                        },
                        Cmd::DirDown => { // move selection in child dir
                            if let Some(child_entries) = &cur_dir.child_entries {
                                if cur_dir.selected_entry_index < child_entries.len() - 1 {
                                    cur_dir.selected_entry_index += 1;
                                }
                            }
                        },
                        Cmd::DirOut => { // navigate to parent
                            if let Some(parent_path) = &cur_dir.parent_path {
                                let parent_dir = Dir::new(parent_path.to_path_buf()).await?;
                                cur_dir = parent_dir;
                            }
                        },
                        Cmd::DirIn => { // navigate to selected child
                            if let Some(_child_entries) = &cur_dir.child_entries {
                                let selected_dir = Dir::new(_child_entries[cur_dir.selected_entry_index].path().to_path_buf()).await?;

                                if let Ok(metadata) = &selected_dir.path.metadata() {
                                    if !metadata.is_file() {
                                        cur_dir = selected_dir;
                                    }
                                }
                            }
                        },
                    }
                },
                _ => {},
            }
            /***                    ***/

            let (width, height) = terminal::size()?;
            let (x, mut y): (u16, u16) = (0, 0);
            let highlight_color = style::Color::Blue;

            queue!(
                w,
                style::ResetColor,
                cursor::MoveTo(x, y),
                cursor::Hide,
                terminal::Clear(terminal::ClearType::All)
            )?;

            // TITLE //
            let cur_dir_str = format!("{}", &cur_dir.path.to_str().unwrap());
            queue!(w, style::Print(cur_dir_str))?;
            y += 1;

            // PANE ONE //
            {
                let (x, mut y): (u16, u16) = (0, y + 1);
                queue!(w, cursor::MoveTo(x, y))?; 

                // print parent dir
                if let Some(parent_path) = &cur_dir.parent_path {
                    let parent_dir = Dir::new(parent_path.to_path_buf()).await?;
                    // existence guaranteed
                    let child_entries = parent_dir.child_entries.unwrap();

                    for (i, child) in child_entries.iter().enumerate() {
                        if i > height as usize {
                            break;
                        }
                        queue!(w, cursor::MoveTo(x, y))?;

                        let selected = child.path() == cur_dir.path;

                        // always select cwd on parentside pane
                        if selected {
                            queue!(w, style::SetBackgroundColor(highlight_color))?;
                        }

                        let name = child.file_name();
                        let name = name.to_str().unwrap();

                        let mut fmt = format!("{}", &name);
                        let len = name.len();
                        if (len as u16 + width/2) > width {
                            fmt.truncate((width/2) as usize);
                        }
                        queue!(w, style::Print(fmt))?;

                        if selected {
                            let n = width/2 - len as u16;
                            if n > 0 {
                                queue!(w, style::ResetColor, style::SetBackgroundColor(highlight_color))?;

                                for _ in 0..n-1 {
                                    queue!(w, style::Print(" "))?;
                                }
                                queue!(w, style::ResetColor)?;
                            }
                        }

                        y += 1;
                    }
                }
            }

            // PANE TWO //
            {
                let (x, mut y): (u16, u16) = (width / 2, y + 1);
                queue!(w, cursor::MoveTo(x, y))?; 


                if let Some(child_entries) = &cur_dir.child_entries {
                    let start: usize = 0;
                    let mut end: usize = (height - 2) as usize;
                    if child_entries.len() < end {
                        end = child_entries.len();
                    }

                    let slice = &child_entries[start..end];
                    for (i, child) in slice.iter().enumerate() { 
                        if i > height as usize {
                            break;
                        }

                        queue!(w, cursor::MoveTo(x, y))?;

                        let selected = i == cur_dir.selected_entry_index;

                        let name = child.file_name();
                        let name = name.to_str().unwrap();

                        if selected {
                            queue!(w, style::SetBackgroundColor(highlight_color))?;
                        }

                        // need to truncate string dep on term width
                        let mut fmt = format!("{}", &name);
                        let len = name.len();
                        if (len as u16 + width/2) > width {
                            fmt.truncate((width/2) as usize);
                        }
                        queue!(w, style::Print(fmt))?;

                        // draw highlight bar for width of terminal
                        if selected {
                            let n = width/2 - len as u16;
                            if n > 0 {
                                queue!(w, style::ResetColor, style::SetBackgroundColor(highlight_color))?;

                                for _ in 0..n-1 {
                                    queue!(w, style::Print(" "))?;
                                }
                                queue!(w, style::ResetColor)?;
                            }
                        }

                        y += 1;
                    }
                }
            }

            w.flush()?;
        }
    }

    release_buffer(&mut w)
    /////////////////////////////////////////////////////
}

fn init_buffer<W>(w: &mut W) -> Result<()> where W: Write {
    execute!(
        w,
        style::ResetColor,
        cursor::Hide,
        terminal::EnterAlternateScreen
    )?;
    terminal::enable_raw_mode()
}

fn release_buffer<W>(w: &mut W) -> Result<()> where W: Write {
    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?; 
    terminal::disable_raw_mode()
}
