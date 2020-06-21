use std::io::stdout;
use crossterm::Result;

/// crate imports/modules
mod renderer;

mod filer;

mod input;

pub async fn init() -> Result<()> {
    // every time a new dir is attempted to be accessed
    // via pathbuf, it will try to add it into the hashmap.
    // inserting to a hashmap returns an Option<V>, None meaning that it was
    // a new key, and Some meaning that the key existed & value was updated. the key/value pairs
    // will never change. or just use contains_key
    //
    // In regards to thread safety, this hashmap is only being used
    // by the renderer, so there are no issues there.
    //let mut dir_cache: HashMap<PathBuf, Dir> = HashMap::new();
    // Insert cwd on startup
    //dir_cache.insert(env::current_dir()?, Dir::new(env::current_dir()?).await?);

    //input::run(inp_txr, f_chords).await;

    renderer::run(stdout()).await?;

    // tokio::spawn(input_handler(sender_channel)) -> watch channel
    // tokio::spawn(renderer) -> recv from FS 
    // tokio::spawn(fs) -> send to renderer, receive from input
    //      - holds info about dir tree/caches
    Ok(())
}
