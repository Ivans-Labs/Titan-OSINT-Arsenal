use std::error;

// Import the UsernameSearcher module
mod usernamesearcher;
use usernamesearcher::UsernameSearcher;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    /// Information gathered by the OSINT tool
    pub osint_data: Vec<String>,
    /// Username searcher
    pub username_searcher: UsernameSearcher,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            osint_data: Vec::new(),
            username_searcher: UsernameSearcher::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of `App`.
    pub fn new() -> Self {
        Self::default()
    }

    
    /// Handles the tick event of the terminal.
    pub fn tick(&self) {
        // Perform any necessary actions on each tick event
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Search for a username and add the result to the OSINT data.
    pub fn search_username(&mut self, username: &str) -> AppResult<()> {
        let result = self.username_searcher.search_username(username)?;
        self.osint_data.push(result);
        Ok(())
    }
}