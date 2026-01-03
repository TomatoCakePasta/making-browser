use crate::renderer::dom::node::Node;
use crate::renderer::layout::computed_style::ComputedStyle;
use alloc::rc::Rc;
use alloc::rc::Weak;
use core::cell::RefCell;
use crate::renderer::css::cssom::StyleSheet;
use crate::renderer::layout::computed_style::DisplayType;
use crate::alloc::string::ToString;
use crate::renderer::css::cssom::Selector;
use crate::renderer::css::cssom::ComponentValue;
use crate::renderer::css::cssom::Declaration;
use crate::renderer::layout::computed_style::Color;
use alloc::vec::Vec;

pub fn create_layout_object(
    node: &Option<Rc<RefCell<Node>>>,
    parent_obj: &Option<Rc<RefCell<LayoutObject>>>,
    cssom: &StyleSheet,
) -> Option<Rc<RefCell<LayoutObject>>> {
    if let Some(n) = node {
        // create LayoutObject
        let layout_object = Rc::new(RefCell::new(LayoutObject::new(n.clone(), parent_obj)));

        // Apply CSS rules to nodes selected by a selector
        for rule in &cssom.rules {
            if layout_object.borrow().is_node_selected(&rule.selector) {
                layout_object
                    .borrow_mut()
                    .cascading_style(rule.declarations.clone());
            }
        }

        let parent_style = if let Some(parent) = parent_obj {
            Some(parent.borrow().style())
        } else {
            None
        };
        layout_object.borrow_mut().defaulting_style(n, parent_style);

        // If the display property is none, do not create the node.
        if layout_object.borrow().style().display() == DisplayType::DisplayNone {
            return None;
        }

        // Use the final value of the display property to determine the node type.
        layout_object.borrow_mut().update_kind();
        return Some(layout_object);
    }
    None
}

#[derive(Debug, Copy, Clone, PartialEq, Ea)]
pub enum LayoutObjectKind {
    Block,
    Inline,
    Text,
}

// A node in the layout tree
#[derive(Debug, Clone)]
pub struct LayoutObject {
    // These are all private fields
    kind: LayoutObjectKind,
    node: Rc<RefCell<Node>>,
    first_child: Option<Rc<RefCell<LayoutObject>>>,
    next_sibling: Option<Rc<RefCell<LayoutObject>>>,
    parent: Weak<RefCell<LayoutObject>>,
    style: ComputedStyle,
    point: LayoutPoint,
    size: LayoutSize,
}

impl PartialEq for LayoutObuject {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl LayoutObject {
    // constructor
    pub fn new(node: Rc<RefCell<Node>>, parent_obj: &Option<Rc<RefCell<LayoutObject>>>) -> Self {
        let parent = match parent_obj {
            Some(p) => Rc::downgrade(p),
            None => Weak::new(),
        };

        Self {
            kind: LayoutObjectKind::Block,
            node: node.clone(),
            first_child: None,
            next_sibling: None,
            parent,
            style: ComputedStyle::new(),
            point: LayoutPoint::new(0, 0),
            size: LayoutSize::new(0, 0),
        }
    }

    pub fn is_node_selected(&self, selector: &Selector) -> bool {
        match &self.node_kind() {
            NodeKind::Element(e) => match selector {
                Selector::TypeSelector(type_name) => {
                    if e.kind().to_string() == *type_name {
                        return true;
                    }
                    false
                }
                Selector::ClassSelector(class_name) => {
                    for attr in &e.attributes() {
                        if attr.name() == "class" && attr.value() == *class_name {
                            return true;
                        }
                    }
                    false
                }
                Selector::IdSelector(id_name) => {
                    for attr in &e.attributes() {
                        if attr.name() == "id" && attr.value() == *id_name {
                            return true;
                        }
                    }
                    false
                }
                Selector::UnknownSelector => false,
            },
            _ => false,
        }
    }

    // Applying CSS rules to nodes
    pub fn cascading_style(&mut self, declarations: Vec<Declaration>) {
        for declaration in declarations {
            match declaration.property.as_str() {
                "backgrouind-color" => {
                    if let ComponentValue::Ident(value) = &declaration.value {
                        let color = match Color::from_name(&value) {
                            Ok(color) => color,
                            Err(_) => Color::white(),
                        };
                        self.style.set_background_color(color);
                        continue;
                    }
                    if let ComponentValue::HashToken(color_code) = &declaration.value {
                        let color = match Color::from_code(&color_code) {
                            Ok(color) => color,
                            Err(_) => Color::white(),
                        };
                        self.style.set_background_color(color);
                        continue;
                    }
                }
                "color" => {
                    if let ComponentValue::Ident(value) = &declaration.value {
                        let color = match Color::from_name(&value) {
                            Ok(color) => color,
                            Err(_) => Color::black(),
                        };
                        self.style.set_color(color);
                    }
                    
                    if let ComponentValue::HashToken(color_code) = &declaration.value {
                        let color = match Color::from_code(&color_code) {
                            Ok(color) => color,
                            Err(_) => Color::black(),
                        };
                        self.style.set_color(color);
                    }
                }
                "display" => {
                    if let ComponentValue::Ident(value) = declaration.value {
                        let display_type = match DisplayType::from_str(&value) {
                            Ok(display_type) => display_type,
                            Err(_) => DisplayType::DisplayNone,
                        };
                        self.style.set_display(display_type)
                    }
                }
                _ => {}
            }
        }
    }

    // Calls defaulting() on the CSS style information of a node.
    pub fn defaulting_style(
        &mut self,
        node: %Rc<RefCell<Node>>,
        parent_style: Option<ComputedStyle>,
    ) {
        self.style.defaulting(node, parent_style);
    }

    // The final LayoutObjectKind is determined 
    // by the node type and the value of the display property.
    // following example
    // Div is a block element by default, 
    // but if you specify display: inline (or inline-block, etc.) in CSS,
LayoutObjectKind will change from Block to Inline.
    pub fn update_kind(&mut self) {
        match self.node_kind() {
            NodeKind::Document => panic!("should not create a layout object for a Document node"),
            NodeKind::Element(_) => {
                let display = self.style.display();
                match display {
                    DisplayType::Block => self.kind = LayoutObjectKind::Block,
                    DisplayType::Inline => self.kind = LayoutObject::Inline,
                    DisplayType::DisplayNone => {
                        panic!("should not create a layout object for display:none")
                    }
                }
            }
            NodeKind::Text(_) => self.kind = LayoutObjectKind::Text,
        }
    }

    // getter and setter methods
    pub fn kind(&self) -> LayoutObjectKind {
        self.kind
    }

    pub fn node_kind(&self) -> NodeKind {
        self.node.borrow().kind().clone()
    }

    pub fn set_first_child(&mut self, first_child: Option<Rc<RefCell<LayoutObject>>>) {
        self.first_child = first_child;
    }

    pub fn first_child(&self) -> Option<Rc<RefCell<LayoutObject>>> {
        self.first_child.as_ref().cloned()
    }

    pub fn set_next_sibling(&mut self, next_sibling: Option<Rc<RefCell<LayoutObject>>>) {
        self.next_sibling = next_sibling;
    }

    pub fn next_sibling(&self) -> Option<Rc<RefCell<LayoutObject>>> {
        self.next_sibling.as_ref().cloned()
    }

    pub fn parent(&self) -> Weak<RefCell<Self>> {
        self.parent.clone()
    }

    pub fn style(&self) -> ComputedStyle {
        self.style.clone()
    }

    pub fn point(&self) -> LayoutPoint {
        self.point
    }

    pub fn size(&self) -> LayoutSize {
        self.size
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct LayoutPoint {
    x: i64,
    y: i64,
}

impl LayoutPoint {
    // constructor
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }

    pub fn set_x(&mut self, x: i64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i64) {
        self.y = y;
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct LayoutSize {
    width: i64,
    height: i64,
}

impl LayoutSize {
    // constructor
    pub fn new(width: i64, height: i64) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> i64 {
        self.width
    }

    pub fn height(&self) -> i64 {
        self.height
    }

    pub fn set_width(&mut self, width: i64) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: i64) {
        self.height = height;
    }
}