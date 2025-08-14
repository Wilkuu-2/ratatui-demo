use std::{io::{Stdout, Write}, time::Duration};

use crossterm::event::Event;
use ratatui::{prelude::CrosstermBackend, Terminal};
use futures::{future::FutureExt, StreamExt};

mod app;
use app::MyApp;
use terminput::KeyCode;
use terminput_crossterm::to_terminput;
use termion::raw::{IntoRawMode, RawTerminal};

pub type Term = Terminal<CrosstermBackend<RawTerminal<Stdout>>>;

trait Demo: Send + Sized + 'static {
    /// Needed to initialize state
    fn new() -> Self; 

    /// Called to draw a frame
    async fn frame(&mut self, term: &mut Term) -> Result<(), String>;

    /// Called when user puts down input 
    async fn input(&mut self, event: terminput::Event) -> Result<(), String>; 

    /// Called on cleanup 
    async fn disconnected(&mut self); 
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    let mut raw_term = std::io::stdout().into_raw_mode()?;
    // Open alternate buffer
    raw_term.write(b"\x1b[?1049h")?;
    let mut term = Terminal::new(CrosstermBackend::new(raw_term))?; 
    let mut app = MyApp::new();
     
    let mut input = crossterm::event::EventStream::new();
    let mut frame_timer = tokio::time::interval(Duration::new(0, 16666)); // 16ms -> 60fps
    

    loop { 
        tokio::select! {
            ev = input.next().fuse() => if sh_input(ev, &mut app).await? {
                app.disconnected().await;
                break;
            }, 
            _ = frame_timer.tick() => app.frame(&mut term).await.map_err(err_map)?,
            _ = tokio::signal::ctrl_c() => {
                app.disconnected().await;
                break;
            }, 
        }
    }

    // Clear screen and close the alternate buffer
    term.backend_mut().write(b"\x1b[2J\x1b[?1049l")?; 
    Ok(())
}

async fn sh_input<A: Demo>(ev: Option<Result<Event, std::io::Error>>, app: &mut A) -> Result<bool, std::io::Error> {
    match ev {
        Some(Ok(event)) => { 
            let input = to_terminput(event).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            if let Some(k) = input.as_key() && k.code == KeyCode::Enter {
                // Exit on enter (Standard for demos)
                Ok(true)
            } else {
                app.input(input).await.map(|_| false).map_err(err_map)
            } 

            
        }
        Some(Err(e)) => Err(e), 
        None => Ok(true), 
    }
}

fn err_map(e: String) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, e)
}


