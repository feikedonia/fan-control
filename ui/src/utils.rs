#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MyOption<T>
where
    T: ToString,
{
    Some(T),
    None,
}

impl<T> ToString for MyOption<T>
where
    T: ToString,
{
    fn to_string(&self) -> String {
        match self {
            MyOption::Some(t) => t.to_string(),
            MyOption::None => fl!("none"),
        }
    }
}

impl<T> From<MyOption<T>> for Option<T>
where
    T: ToString,
{
    fn from(value: MyOption<T>) -> Self {
        match value {
            MyOption::Some(value) => Some(value),
            MyOption::None => None,
        }
    }
}
pub mod input {

    use data::{
        app_graph::Nodes,
        id::Id,
        node::{Input, Node},
    };

    use super::MyOption;

    impl From<String> for MyOption<Input> {
        fn from(value: String) -> Self {
            MyOption::Some(Input {
                id: Id::default(),
                name: value,
            })
        }
    }

    impl From<Option<String>> for MyOption<Input> {
        fn from(value: Option<String>) -> Self {
            match value {
                Some(value) => MyOption::Some(Input {
                    id: Id::default(),
                    name: value,
                }),
                None => MyOption::None,
            }
        }
    }

    /// Return an iter of all inputs availlable for this node, minus his inputs
    pub fn availlable_inputs<'a>(
        nodes: &'a Nodes,
        node: &'a Node,
    ) -> impl Iterator<Item = Input> + 'a {
        nodes
            .values()
            .filter(|n| {
                node.node_type
                    .allowed_dep()
                    .contains(&n.node_type.to_light())
                    && !node
                        .inputs
                        .iter()
                        .map(|i| i.id)
                        .collect::<Vec<_>>()
                        .contains(&n.id)
            })
            .map(|n| Input {
                id: n.id,
                name: n.name().clone(),
            })
    }

    pub fn optional_availlable_inputs<'a>(
        nodes: &'a Nodes,
        node: &'a Node,
        add_node: bool,
    ) -> Vec<MyOption<Input>> {
        let mut vec: Vec<MyOption<Input>> = if add_node {
            vec![MyOption::None]
        } else {
            Vec::new()
        };

        let values = nodes
            .values()
            .filter(|n| {
                node.node_type
                    .allowed_dep()
                    .contains(&n.node_type.to_light())
                    && !node
                        .inputs
                        .iter()
                        .map(|i| i.id)
                        .collect::<Vec<_>>()
                        .contains(&n.id)
            })
            .map(|n| {
                MyOption::Some(Input {
                    id: n.id,
                    name: n.name().clone(),
                })
            });

        vec.extend(values);
        vec
    }
}

pub mod hardware {

    use std::rc::Rc;

    use hardware::HItem;

    use super::MyOption;

    /// Return hardware info about `hardware_id` and a vec of
    /// availlable hardware
    pub fn availlable_hardware<'a>(
        hardware_id: &'a Option<String>,
        hardwares: &'a [Rc<HItem>],
        one_ref: bool,
    ) -> (MyOption<HItem>, Vec<MyOption<HItem>>) {
        let mut selected_hardware_info: MyOption<HItem> = MyOption::None;

        let mut hardware_options: Vec<_> = hardwares
            .iter()
            .filter_map(|h| {
                let is_equal = match hardware_id {
                    Some(hardware_id) => {
                        if hardware_id == &h.hardware_id {
                            selected_hardware_info = MyOption::Some(HItem::clone(h));
                            true
                        } else {
                            false
                        }
                    }
                    None => false,
                };

                if one_ref {
                    // we leverage rc to know if this specific hardware
                    // is already in use by one node
                    if Rc::strong_count(h) > 1 {
                        return None;
                    }
                }

                // we only add if hardware_id != h
                if !is_equal {
                    Some(MyOption::Some(HItem::clone(h)))
                } else {
                    None
                }
            })
            .collect();

        if hardware_id.is_some() {
            hardware_options.insert(0, MyOption::None);
        }

        (selected_hardware_info, hardware_options)
    }
}
