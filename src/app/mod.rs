/***************************************************************
**
** NanoKit Library Source File
**
** File         :  mod.rs
** Module       :  app
** Crate        :  NanoUI
** Project      :  NanoKit
** Author       :  SH
** Description  :  Contains the NanoUI Application interface.
**
***************************************************************/

/***************************************************************
* MARK: EXTERNAL MODULES
***************************************************************/

pub use crate::Application;
pub use crate::Window;

use std::cell::RefCell;

/***************************************************************
* MARK: CHILD MODULES
***************************************************************/

/***************************************************************
* MARK: TYPE DEFINITIONS
***************************************************************/

pub trait ApplicationDelegate
{
    // called when the application has finished launching
    fn application_launched(&mut self, app: Application);
    
    // called when the application is about to terminate (e.g. user closed the app)
    fn application_will_terminate(&mut self, app: Application);

    // called when the application is activated (e.g. user switched to the app)
    fn application_activated(&mut self, app: Application);

    // called when the application is deactivated (e.g. user switched away from the app)
    fn application_deactivated(&mut self, app: Application);
}

pub enum ApplicationQuitMode
{
    OnLastWindowClose,
    OnMainWindowClose,
    OnManualQuit
}

struct ApplicationState
{
    pub title: String,
    pub description: String,
    pub developer: String,

    pub quit_mode: ApplicationQuitMode
}

/***************************************************************
* MARK: STATIC VARIABLES & CONSTANTS
***************************************************************/

thread_local! {
    static APP_STATE: RefCell<ApplicationState> = RefCell::new(
        ApplicationState {
            title: String::from(""),
            description: String::from(""),
            developer: String::from(""),
            quit_mode: ApplicationQuitMode::OnLastWindowClose
        }
    );
}

/***************************************************************
* MARK: PUBLIC FUNCTIONS
***************************************************************/

pub fn run() -> i32
{
    return nanowin::platform::platform::run();
}


/***************************************************************
* MARK: LOCAL FUNCTIONS
***************************************************************/