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

pub fn get_element_string(elem: &html_parser::Element, deep: bool) -> String {
    let t = elem
        .children
        .clone()
        .into_iter()
        .filter_map(|c| match c {
            html_parser::Node::Text(t) => Some(t),
            html_parser::Node::Element(e) => Some(format!(
                " <{}>{}</{}> ",
                e.name,
                get_element_string(&e, true),
                e.name
            )),
            html_parser::Node::Comment(_) => None,
        })
        .collect::<Vec<String>>()
        .join("");

    if deep {
        return t;
    }

    return t.splitn(2, ". ").collect::<Vec<&str>>()[1].to_owned();
}
