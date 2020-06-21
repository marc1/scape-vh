## Why?
Being frustrated with existing terminal file managers (i.e. [broot](https://github.com/Canop/broot) in Rust, [lf](https://github.com/gokcehan/lf) in Go), I decided to make my own. There's nothing wrong with them, just small things that I felt I could do better.

 In its current state, _scape_ is not yet rivalling the functionality of these existing solutions, but serves as a starting point for future development.

For me, developing this was less about the program itself, and more about diving deeper into Rust, and asynchronous Rust programming: I learned a lot from making this.

## What does it do?
_scape_ is a lightweight and fast terminal file manager written in Rust, with asynchronous tasks being multiplexed across all physical CPU cores using [tokio](https://github.com/tokio-rs/tokio) and it's _rt-threaded_ feature. This allows non-blocking reads of directories, files, and keyboard input. 

Instead of a normal GUI, file icons, and mouse input, _scape_ lives entirely in the terminal. It displays text, some color, and allows you to use only a keyboard to navigate.

Being a vim user, I decided to design scape with existing vim bindings in mind. You use **h j k l** to move around, and vim-style key chords are also available. I find that using terminal and command line applications greatly boosts my workflow, as I am able to navigate around my filesystem quicker with a keyboard than with a mouse.

## How does it work?
_scape_ only has two dependencies: 

- [crossterm](https://github.com/crossterm-rs/crossterm) - A cross platform library for terminal manipulation
- [tokio](https://github.com/tokio-rs/tokio) - An fast, asynchronous runtime library

Essentially, [crossterm](https://github.com/crossterm-rs/crossterm) allows us to write text to the terminal, push some colors, and move stuff around how we'd like. [tokio](https://github.com/tokio-rs/tokio) allows for concurrent/parallel tasks, meaning operations that could take a while, like reading a directory or listening for keystrokes, can be ran at the same time.

On the topic of keystrokes, _scape_ is designed for easy implementation of configurations in the future. Take a look at how easy it is to add a new chord (a chord is a sequence of keys pressed to yield an action from the program):
```rust
let f_chords: HashMap<Chord, Cmd> = HashMap::new();
f_chords.insert(Chord('q', 'q'), Cmd::Quit);
```
I wrote it as `f_chord`, meaning functional chord, since it has a "function", or `Cmd` attached to it. This particular chord would require me to press 'qq' to quit _scape_.  The `Cmd` is listened for asynchronously, and when it's detected, a corresponding action is performed. If you're wondering how `Chord` and `Cmd` are defined, they're quite simple:

```rust
#[derive(Copy, Clone, Debug)]
pub enum Cmd {
    Quit, 		// Leave the program
    DirUp, 		// Scroll up
    DirDown, 	// Scroll down
    DirOut, 	// Go to the parent directory
    DirIn 		// Go into the directory you're highlighting
}

#[derive(Eq, Hash)]
pub struct Chord(pub char, pub char);
```
The derivations on `Cmd` are there because of how I designed _scape_ around Rust's type system. The derivations on `Chord` are useful because it allows us to use `HashMap`s and `HashSet`s.  These collections are great because they offer fast lookup, and all keys are guaranteed to be unique. This means we don't allow any duplicate `Chord` sequences, or any duplicate directories in our cache.

## Challenges
While I've tinkered with Rust in the past, _scape_ is my first real Rust project. Rust's safety is one of its main features, and that can somewhat slow down a developer used to more unsafe languages. On top of that, thinking in/writing in an asynchronous mindset for the first time was definitely a challenge. That being said, [tokio](https://github.com/tokio-rs/tokio) was fantastic to work with, and I'll definitely be using it to continue _scape_'s development.

## Considerations for the future
Here's a quick list of things I plan to consider for continuing _scape_:

- **Leverage Rust's ownership model and type system more** - I didn't really make full use of it in this project, given the short period of time and how new I am to Rust, but I am definitely interested in learning Rust more depth and applying it to _scape_.
- **Segregate [tokio](https://github.com/tokio-rs/tokio) tasks** - At the moment, input handling and rendering are sort of intertwined. I'd like to seperate those, along with other functions of the program, to make it easier to work on.
- **Directory caching** - I almost implemented it in this version, but decided against it. My original implementation was a `HashMap` keyed with the `PathBuf` of the directory in question. This theoretically would have been pretty fast, but I'm open to explore more (and potentially better) solutions.
- **Nailing down the filesystem/file tree structure** - I'm not entirely happy with how I structured the filesystem. I think I could have done a better job. With more considerations and time, it should see big improvement.
- **Configuration system** - I'd like users to be able to configure _scape_ to their liking, through a file.
- **Scrolling** - A basic quality of life feature that I did not have time to add. My original idea involved mutating a slice of a `Vec`, which is an illegal operation in Rust. I'll have to rethink my approach.
- **Editor and Opener support** - I'd like to be able to hit a key and open a file in vim, or NotePad, or whatever I choose!
- **File and Image previews** - In an attempt to completely replace my GUI file manager, I'd like to have file and image previews.
- **nvim plugin** - Once _scape_ is in a stable place, I plan on making an `nvim` plugin for it, so that I can use it to navigate around my codebase.

## Bugs
Currently, there is one known bug:

-	**Slight flickering when traversing large directories:** I don't know the cause of this yet. I think it might have something to do with `cursor::Hide` from [crossterm](https://github.com/crossterm-rs/crossterm) not working properly.

## Continuing the project

Right now, you can find _scape_ at [https://github.com/marc1/scape-vh](https://github.com/marc1/scape-vh), `vh` being for [VikesHack]([https://vikehacks-2020.devpost.com/](https://vikehacks-2020.devpost.com/)). I'll be continuing the project at https://github.com/marc1/scape in the future.

