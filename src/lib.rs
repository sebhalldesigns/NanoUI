

pub use std::rc::Rc;
use std::vec::Vec;

// MARK: PARENT LAYOUT

pub enum ParentLayout 
{
    None,
    Dock(DockOptions),
    Stack(StackOrientation),
    Grid
}

pub struct DockOptions 
{
    pub last_child_fill: bool
}

pub struct StackOptions 
{
    pub orientation: StackOrientation,
    pub spacing: f32
}

pub enum StackOrientation 
{
    Horizontal,
    Vertical
}

pub struct GridOptions 
{
    pub rows: Vec<GridMeasurement>,
    pub columns: Vec<GridMeasurement>
}

pub enum GridMeasurement 
{
    Auto,
    Star(GridMeasurementValue),
    Pixel(GridMeasurementValue)
}

pub struct GridMeasurementValue 
{
    pub value: f32,
    pub min: Option<f32>,
    pub max: Option<f32>
}

// MARK: CHILD LAYOUT

pub enum DockPosition
{
    Top,
    Bottom,
    Left,
    Right
}

pub struct GridPosition 
{
    pub row: usize,
    pub column: usize,
    pub row_span: usize,
    pub column_span: usize
}

pub enum HorizontalAlignment 
{
    Fill,
    Left,
    Center,
    Right
}

pub enum VerticalAlignment 
{
    Fill,
    Top,
    Center,
    Bottom
}

pub struct Thickness
{
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32
}

pub struct Rect 
{
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}

pub struct Size 
{
    pub width: f32,
    pub height: f32
}

pub struct View
{
    pub parent: Option<Rc<RefCell<View>>>,
    pub children: Vec<Rc<RefCell<View>>>,


    pub layout: ParentLayout,

    pub dock_position: DockPosition,
    pub grid_position: GridPosition,
    pub horizontal_alignment: HorizontalAlignment,
    pub vertical_alignment: VerticalAlignment,

    pub size: Size,    
    pub margin: Thickness,
    pub padding: Thickness,
    pub horizontal_alignment: HorizontalAlignment,
    pub vertical_alignment: VerticalAlignment,

    pub calculated_rect: Rect
}



impl View 
{
    // Create a new parent view
    pub fn new(layout: ParentLayout) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(View {
            parent: None,
            children: Vec::new(),
            layout,
            dock_position: None,
            grid_position: None,
            horizontal_alignment: HorizontalAlignment::Fill,
            vertical_alignment: VerticalAlignment::Fill,
            size: Size { width: 0.0, height: 0.0 },
            margin: Thickness { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 },
            padding: Thickness { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 },
            calculated_rect: Rect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 },
        }))
    }

    // Add a child View
    pub fn add_child(parent: &Rc<RefCell<Self>>, child: Rc<RefCell<View>>) {
        child.borrow_mut().parent = Some(Rc::clone(parent));
        parent.borrow_mut().children.push(child);
    }

    // Calculate layout (basic placeholder logic)
    pub fn calculate_layout(&mut self) {
        // Basic layout logic: size is based on children
        for child in &self.children {
            let mut child_borrowed = child.borrow_mut();
            child_borrowed.calculated_rect = Rect {
                x: self.calculated_rect.x + self.margin.left + self.padding.left,
                y: self.calculated_rect.y + self.margin.top + self.padding.top,
                width: self.size.width - self.margin.left - self.margin.right,
                height: self.size.height - self.margin.top - self.margin.bottom,
            };
            child_borrowed.calculate_layout(); // Recursive layout calculation
        }
    }

    pub fn render(&self) {
        // Placeholder render logic
        for child in &self.children {
            child.borrow().render(); // Recursive render
        }
    }
    

}