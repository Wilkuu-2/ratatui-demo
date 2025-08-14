use std::f64::consts::PI;
use ratatui::{layout::Rect, style::{Style, Stylize}, text::{Line, Span}, widgets::{Block, Paragraph}};
use terminput::KeyCode;

use crate::Demo;

pub(crate) struct MyApp {
    // Put in any variables here 
    counter: usize, 
    frame_counter: usize,
}
// Implement the [`crate::Demo`] trait to outline the behaviour
impl Demo for MyApp {
    fn new() -> Self {
        Self { 
            counter: 0,
            frame_counter: 0,
        }
    }

    async fn frame(&mut self, term: &mut crate::Term) -> Result<(), String> {
        // This example draws two moving sine waves on the screen. 
        // Put in your behaviour here

        // Draw function
        term.draw(|fr| {
            self.frame_counter = self.frame_counter.wrapping_add(1);

            // Color and the draw area 
            let style = Style::new().light_yellow().on_black(); 
            let area = fr.area(); 

            // Create a block with title and credits 
            let block = Block::bordered()
                .style(style)
                .title("Example demo: Sine")
                .title_bottom("By: wilkuu (Jakub Stachurski), jakub@snt.utwente.nl");

            // Area inside of the block
            let barea = block.inner(area);
            
            // Draw the block
            fr.render_widget(block, area);

            // Demo drawing logic replace with own stuff 
            let xl = barea.x; 
            let xr = barea.width + barea.x;

            let yt = barea.y + barea.height;

            let w1 = PI * 2.0 / 100.0; 
            let w2 = PI * 2.0 / 300.0; 

            let buf = fr.buffer_mut();
            for x in xl..xr {
                let p1 = self.frame_counter as f64 / 100.0; 
                let p2 = self.frame_counter as f64 / 50.0; 

                let y1: u16 = ((
                    ((w1 * Into::<f64>::into(x)) + p1 as f64).sin() + 1.0) 
                    * Into::<f64>::into(barea.height / 2-1) + Into::<f64>::into(barea.y)).round() as u16; 

                let y2: u16 = ((
                    ((w2 * Into::<f64>::into(x)) + p2 as f64).sin() + 1.0) 
                    * Into::<f64>::into(barea.height / 2-1) + Into::<f64>::into(barea.y)).round() as u16; 

               if y2 == y1 {
                   // Individual character drawing, there is probably a better way to achieve this.
                   buf.set_span(x, (y1.saturating_sub(1)).max(barea.y), &Span::from("_").light_magenta(), 1);  
                   buf.set_span(x, y1, &Span::from("#").light_magenta(), 1);  
                   buf.set_span(x, (y1.saturating_add(1)).min(yt), &Span::from("-").light_magenta(), 1);  
               } else {
                   buf.set_span(x, (y1.saturating_sub(1)).max(barea.y), &Span::from("_").red(), 1);  
                   buf.set_span(x, y1, &Span::from("#").red(), 1);  
                   buf.set_span(x, (y1.saturating_add(1)).min(yt), &Span::from("-").red(), 1);  

                   buf.set_span(x, (y2.saturating_sub(1)).max(barea.y), &Span::from("_").blue(), 1);  
                   buf.set_span(x, y2, &Span::from("#").blue(), 1);  
                   buf.set_span(x, (y2.saturating_add(1)).min(yt), &Span::from("-").blue(), 1);  
               }
            }
            
            // Center text 
            let mut lines: Vec<Line>= Vec::new(); 
            lines.push("Use W/S to change the counter. Enter to exit.".into());
            lines.push(format!("Counter: {}", self.counter).into());

            let text = Paragraph::new(lines).centered().style(style);
            
            fr.render_widget(text, Rect::new(barea.width/2 - 25, barea.height/2, 50, 4));
        }).map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn input(&mut self, event: terminput::Event) -> Result<(), String> {
        // Handle input using terminput.
        if let terminput::Event::Key(kv) = event {
            match kv.code {
               KeyCode::Char('w') => {
                    self.counter = self.counter.wrapping_add(1);
                },
               KeyCode::Char('s') => {
                    self.counter = self.counter.saturating_sub(1);
                },
                _=> {}
            }
             
        }
        Ok(())
    }

    async fn disconnected(&mut self) {
        ()
    }
}
