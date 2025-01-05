/***************************************************************
**
** NanoKit Library Source File
**
** File         :  mod.rs
** Module       :  window
** Crate        :  NanoUI
** Project      :  NanoKit
** Author       :  SH
** Description  :  Contains the NanoUI Window interface.
**
***************************************************************/

/***************************************************************
* MARK: EXTERNAL MODULES
***************************************************************/

pub use nanowin::{Size, Point, Rect, debug::*};

pub use crate::Window;
pub use crate::View;

use std::collections::HashMap;
use std::cell::RefCell;


/***************************************************************
* MARK: CHILD MODULES
***************************************************************/

/***************************************************************
* MARK: TYPE DEFINITIONS
***************************************************************/

pub trait WindowDelegate
{
    
    fn window_loaded(&mut self, window: Window);
    fn window_resized(&mut self, window: Window, size: Size);
    fn window_mouse_moved(&mut self, window: Window, point: Point);
    fn window_mouse_clicked(&mut self, window: Window, point: Point);
}

struct WindowObject
{
    pub id: Window,

    pub title: String,
    pub size: Size,
    pub mouse_pos: Point,

    pub system_window: nanowin::Window,

    pub root_view: Option<View>
}

struct WindowState
{
    pub windows: HashMap<Window, WindowObject>,
}

/***************************************************************
* MARK: STATIC VARIABLES & CONSTANTS
***************************************************************/

thread_local! {
    static WINDOW_STATE: RefCell<WindowState> = RefCell::new(WindowState {
        windows: HashMap::new(),
    });
}

const DEFAULT_WINDOW_TITLE: &str = "NanoUI Window"; 
const DEFAULT_WINDOW_SIZE: Size = Size { width: 800.0, height: 600.0 };

/***************************************************************
* MARK: PUBLIC FUNCTIONS
***************************************************************/

pub fn create() -> Window
{
    let new_window = WINDOW_STATE.with(|state| {
        let mut state = state.borrow_mut();

        let new_system_window: nanowin::Window = nanowin::window::create(String::from(DEFAULT_WINDOW_TITLE), DEFAULT_WINDOW_SIZE);

        let new_view = WindowObject {
            id: new_system_window,
            title: String::from(""),
            size: DEFAULT_WINDOW_SIZE,
            mouse_pos: Point { x: 0.0, y: 0.0 },
            system_window: new_system_window,
            root_view: None
        };
        
        state.windows.insert(new_view.id, new_view);

        return new_system_window;
    });

    nanowin::window::set_resize_handler(new_window, resize);
    nanowin::window::set_render_handler(new_window, render);

    return new_window;
}

pub fn destroy(window: Window)
{
    log(LogLevel::WARN, &format!("Window destruction not implemented!"));
}

pub fn set_root_view(window: Window, view: View)
{
    WINDOW_STATE.with(|state| {
        let mut state = state.borrow_mut();

        if let Some(window_object) = state.windows.get_mut(&window)
        {
            window_object.root_view = Some(view);
        }
    });

    crate::view::set_window_down(view, Some(window));
}

pub fn resize(window: Window, rect: Rect)
{
    WINDOW_STATE.with(|state| {
        let mut state = state.borrow_mut();

        if let Some(window_object) = state.windows.get_mut(&window)
        {
            if let Some(root_view) = window_object.root_view
            {
                crate::view::layout_down(root_view, rect);
            }
        }
    });
}

pub fn render(window: Window)
{
    WINDOW_STATE.with(|state| {
        let mut state = state.borrow_mut();

        if let Some(window_object) = state.windows.get_mut(&window)
        {
            if let Some(root_view) = window_object.root_view
            {
                crate::view::render_down(root_view);
            }
        }
    });

    let window_size = nanowin::window::get_window_size(window);

    nanowin::renderer::draw_label(window, Rect { origin: Point { x: 10.0, y: 10.0 }, size: Size { width: 250.0, height: 50.0 } }, nanowin::COLOR_GREEN, "Hello, NanoUI!");
}

pub fn mouse_move(window: Window, point: Point)
{
    
}

pub fn mouse_click(window: Window, point: Point)
{
    
}


/***************************************************************
* MARK: LOCAL FUNCTIONS
***************************************************************/