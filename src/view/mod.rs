/***************************************************************
**
** NanoKit Library Source File
**
** File         :  mod.rs
** Module       :  view
** Crate        :  NanoUI
** Project      :  NanoKit
** Author       :  SH
** Description  :  Contains the NanoUI View interface
**
***************************************************************/

/***************************************************************
* MARK: EXTERNAL MODULES
***************************************************************/

use nanowin::{Window, COLOR_BLUE};
pub use nanowin::{Size, Point, Color, Rect, debug::*};

pub use crate::View;

use std::collections::HashMap;
use std::cell::RefCell;

/***************************************************************
* MARK: CHILD MODULES
***************************************************************/

/***************************************************************
* MARK: TYPE DEFINITIONS
***************************************************************/

#[derive(Debug, Clone, Copy)]
pub enum DockLocation
{
    Top,
    Bottom,
    Left,
    Right
}

struct ViewObject
{
    pub id: View,

    pub window: Option<Window>,
    pub parent: Option<View>,
    pub children: Vec<View>,

    pub requested_size: Size,
    pub dock_location: DockLocation,
    pub background_color: Color,

    pub calculated_rect: Rect,

}   

impl Default for ViewObject
{
    fn default() -> Self
    {
        return ViewObject {
            id: 0,
            window: None,
            parent: None,
            children: Vec::new(),
            requested_size: Size { width: 0.0, height: 0.0 },
            dock_location: DockLocation::Left,
            background_color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
            calculated_rect: Rect { origin: Point { x: 0.0, y: 0.0 }, size: Size { width: 0.0, height: 0.0 } }
        };
    }
}

struct ViewState
{
    pub view_id_allocator: View,
    pub views: HashMap<View, ViewObject>
}

/***************************************************************
* MARK: STATIC VARIABLES & CONSTANTS
***************************************************************/

thread_local! {
    static VIEW_STATE: RefCell<ViewState> = RefCell::new(ViewState {
        view_id_allocator: 1,
        views: HashMap::new()
    });
}

/***************************************************************
* MARK: PUBLIC FUNCTIONS
***************************************************************/

pub fn create() -> View
{

    let new_view = VIEW_STATE.with(|state| {
        let mut state = state.borrow_mut();

        let new_view_id = state.view_id_allocator;
        state.view_id_allocator += 1;

        let mut new_view = ViewObject::default();
        new_view.id = new_view_id;
        
        state.views.insert(new_view.id, new_view);

        return new_view_id;
    });

    log(LogLevel::INFO, &format!("Created view with ID: {}", new_view));

    return new_view;
}

pub fn destroy(view: View)
{
    log(LogLevel::WARN, &format!("View destruction not implemented!"));
}

pub fn add_child(parent: View, child: View)
{
    VIEW_STATE.with(|state| {
        let mut state = state.borrow_mut();

        if (state.views.contains_key(&parent) && state.views.contains_key(&child))
        {
            if let Some(child_view) = state.views.get_mut(&child)
            {
                child_view.parent = Some(parent);
            }
            else 
            {
                log(LogLevel::FAIL, &format!("Child view not found!"));
            }
            
            if let Some(parent_view) = state.views.get_mut(&parent)
            {
                parent_view.children.push(child);
            }
            else 
            {
                log(LogLevel::FAIL, &format!("Parent view not found!"));
            }
        }
        else 
        {
            log(LogLevel::FAIL, &format!("Parent or child view not found!"));
        }

    });
}

pub fn set_background_color(view: View, color: Color)
{
    VIEW_STATE.with(|state| {
        let mut state = state.borrow_mut();

        if let Some(view_object) = state.views.get_mut(&view)
        {
            view_object.background_color = color;
        }
        else 
        {
            log(LogLevel::FAIL, &format!("View not found!"));
        }
    });
}

pub fn set_dock_location(view: View, location: DockLocation)
{
    VIEW_STATE.with(|state| {
        let mut state = state.borrow_mut();

        if let Some(view_object) = state.views.get_mut(&view)
        {
            view_object.dock_location = location;
        }
        else 
        {
            log(LogLevel::FAIL, &format!("View not found!"));
        }
    });
}

pub fn get_dock_location(view: View) -> DockLocation
{
    return VIEW_STATE.with(|state| {
        let state = state.borrow();

        if let Some(view_object) = state.views.get(&view)
        {
            return view_object.dock_location;
        }
        else 
        {
            log(LogLevel::FAIL, &format!("View not found!"));
            return DockLocation::Left;
        }
    });
}

pub fn set_size(view: View, size: Size)
{
    VIEW_STATE.with(|state| {
        let mut state = state.borrow_mut();

        if let Some(view_object) = state.views.get_mut(&view)
        {
            view_object.requested_size = size;
        }
        else 
        {
            log(LogLevel::FAIL, &format!("View not found!"));
        }
    });
}

pub fn get_size(view: View) -> Size
{
    return VIEW_STATE.with(|state| {
        let state = state.borrow();

        if let Some(view_object) = state.views.get(&view)
        {
            return view_object.requested_size;
        }
        else 
        {
            log(LogLevel::FAIL, &format!("View not found!"));
            return Size { width: 0.0, height: 0.0 };
        }
    });
}

pub fn render_down(view: View)
{

    // render this view
    render_view(view);

    // render children

    let children_to_render = VIEW_STATE.with(|state| {
        let state = state.borrow();

        if let Some(view_object) = state.views.get(&view)
        {
            return view_object.children.clone();
        }
        else 
        {
            log(LogLevel::FAIL, &format!("View not found!"));
            return Vec::new();
        }
    });

    for child in children_to_render
    {
        render_down(child);
    }
}

pub fn layout_down(view: View, rect: Rect)
{
    //log(LogLevel::INFO, &format!("Resizing view: {}", view));

    set_view_rect(view, rect);

    let children_to_resize = VIEW_STATE.with(|state| {
        let state = state.borrow();

        if let Some(view_object) = state.views.get(&view)
        {
            return view_object.children.clone();
        }
        else 
        {
            log(LogLevel::FAIL, &format!("View not found!"));
            return Vec::new();
        }
    });

    let mut left: f32 = rect.origin.x;
    let mut top: f32 = rect.origin.y;
    let mut right: f32 = rect.origin.x + rect.size.width;
    let mut bottom: f32 = rect.origin.y + rect.size.height;

    for i in 0..children_to_resize.len()
    {

        if i >= children_to_resize.len() - 1
        {
            let child = children_to_resize[i];
            let child_rect = Rect { origin: Point { x: left, y: top }, size: Size { width: right - left, height: bottom - top } };

            layout_down(child, child_rect);
        }
        else 
        {
            let child = children_to_resize[i];

            let child_location = get_dock_location(child);
            let child_size = get_size(child);
            
            let mut child_rect = Rect { origin: Point { x: left, y: top }, size: Size { width: right - left, height: bottom - top } };

            match child_location
            {
                DockLocation::Top => 
                { 
                    child_rect.origin.y = top;
                    child_rect.size.height = child_size.height;
                    top += child_size.height;
                },
                DockLocation::Bottom => 
                { 
                    child_rect.origin.y = top + bottom - child_size.height;
                    child_rect.size.height = child_size.height;
                    bottom -= child_size.height;
                },
                DockLocation::Left => 
                { 
                    child_rect.origin.x = left;
                    child_rect.size.width = child_size.width;
                    left += child_size.width;
                },
                DockLocation::Right => 
                { 
                    child_rect.origin.x = left + right - child_size.width;
                    child_rect.size.width = child_size.width;
                    right -= child_size.width;
                }
            }
            
            layout_down(child, child_rect);
        }

    }

}

pub fn set_window_down(view: View, window: Option<Window>)
{
    let children = VIEW_STATE.with(|state| {
        let mut state = state.borrow_mut();

        if let Some(view_object) = state.views.get_mut(&view)
        {
            view_object.window = window;
            return view_object.children.clone();
        }
        else 
        {
            log(LogLevel::FAIL, &format!("View not found!"));
            return Vec::new();
        }
    });

    for child in children
    {
        set_window_down(child, window);
    }
}


/***************************************************************
* MARK: LOCAL FUNCTIONS
***************************************************************/

fn render_view(view: View)
{
    //log(LogLevel::INFO, &format!("Rendering view: {}", view));

    // render this view
    let window = VIEW_STATE.with(|state| {
        let state = state.borrow();

        if let Some(view_object) = state.views.get(&view)
        {
            return Some((view_object.window, view_object.background_color, view_object.calculated_rect));
        }
        else 
        {
            log(LogLevel::FAIL, &format!("View not found!"));
            return None;
        }
    });

    if window.is_none()
    {
        return;
    }

    let window = window.unwrap();

    if let Some(parent_window) = window.0
    {
        //println!("Drawing rect for view: {}", view);
        nanowin::renderer::draw_rect(parent_window, window.2, window.1);
    }
}   

fn set_view_rect(view: View, rect: Rect)
{
    //log(LogLevel::INFO, &format!("Sizing view: {}", view));

    VIEW_STATE.with(|state| {
        let mut state = state.borrow_mut();

        if let Some(view_object) = state.views.get_mut(&view)
        {
            view_object.calculated_rect = rect;
        }
    });

}