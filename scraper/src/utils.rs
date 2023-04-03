use html_parser::Element;

pub fn find_by_name(elem: &Element, name: &str) -> Element {
    elem.children
        .iter()
        .find(|h| h.element().unwrap().name == name)
        .unwrap()
        .element()
        .unwrap()
        .to_owned()
}

// ??????
pub fn find_by_name_id(elem: &Element, name: &str, id: &str) -> Element {
    elem.children
        .iter()
        .find(|h| {
            h.element().unwrap().name == name && h.element().unwrap().clone().id.unwrap() == id
        })
        .unwrap()
        .element()
        .unwrap()
        .to_owned()
}

// again ??????
pub fn find_by_name_class(elem: &Element, name: &str, classes: Vec<&str>) -> Element {
    elem.children
        .iter()
        .find(|h| {
            h.element().unwrap().name == name && h.element().unwrap().clone().classes == classes
        })
        .unwrap()
        .element()
        .unwrap()
        .to_owned()
}

// again ??????
pub fn find_by_name_class_wo_style(elem: &Element, name: &str, classes: Vec<&str>) -> Element {
    elem.children
        .iter()
        .find(|h| {
            h.element().unwrap().name == name
                && h.element().unwrap().clone().classes == classes
                && !h.element().unwrap().attributes.contains_key("style")
        })
        .unwrap()
        .element()
        .unwrap()
        .to_owned()
}
