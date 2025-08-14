# Ratatui demo. 
A more universal way to draw ratatui applications, made for creating [demo's](https://en.wikipedia.org/wiki/Demoscene).

I created this so people could add demo's to the ssh signon service for SNT.
For more details see [this repo](https://gitlab.snt.utwente.nl/jakub/memadd). 

# How to use. 
Later, I might figure out the cargo template thing, but this would also imply making a library for the trait, which I am too lazy to do right now.

For now the way you can use it is to clone the repo and start writing your demo in [`app.rs`](./src/app.rs). 
If you need to know what each trait method does you can look it up in [`main.rs`](./src/main.rs). 

To run your demo, just type: `cargo run`

# How to share. 
Since right now the whole demo is self-contained, you can just share the links to your repo.
If the person you are sharing with has the same OS and CPU, you can also send the executable in `target/debug/ratatui-demo`, since rust is statically linked. 


