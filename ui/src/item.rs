use data::{node::{Node, NodeType, Nodes}, config::custom_temp::CustomTempKind};
use hardware::Hardware;
use iced::{
    widget::{Button, Column, Container, PickList, Row, Text, TextInput, Toggler},
    Alignment, Element, Length, Padding,
};

use crate::{
    pick::{pick_hardware, pick_input, Pick},
    theme::{CustomContainerStyle, CustomTextInputStyle},
    AppMsg,
};

fn item_view<'a>(node: &'a Node, mut content: Vec<Element<'a, AppMsg>>) -> Element<'a, AppMsg> {
    let mut name =
        TextInput::new("name", &node.name_cached).on_input(|str| AppMsg::Rename(node.id, str));

    if node.is_error_name {
        name = name.style(iced::theme::TextInput::Custom(Box::new(
            CustomTextInputStyle::Error,
        )));
    }

    content.insert(0, name.into());

    let column = Column::with_children(content).spacing(5);

    Container::new(column)
        .width(Length::Fixed(150.0))
        .padding(Padding::new(10.0))
        .style(iced::theme::Container::Custom(Box::new(
            CustomContainerStyle::Item,
        )))
        .into()
}

pub fn control_view<'a>(
    node: &'a Node,
    nodes: &'a Nodes,
    hardware: &'a Hardware,
) -> Element<'a, AppMsg> {
    let NodeType::Control(control) = &node.node_type else {
        panic!()
    };

    let content = vec![
        pick_hardware(node, &hardware.controls, true),
        pick_input(
            node,
            nodes,
            &control.input,
            true,
            Box::new(AppMsg::ReplaceInput),
        ),
        Row::new()
            .push(Text::new(format!("{} %", node.value.unwrap_or(0))))
            .push(Toggler::new(None, !control.auto, |is_active| {
                AppMsg::ChangeControlAuto(node.id, !is_active)
            }))
            // todo: need space_between here
            .align_items(Alignment::End)
            .width(Length::Fill)
            .into(),
    ];

    item_view(node, content)
}

pub fn temp_view<'a>(node: &'a Node, hardware: &'a Hardware) -> Element<'a, AppMsg> {
    let content = vec![
        Text::new(format!("{} °C", node.value.unwrap_or(0))).into(),
        pick_hardware(node, &hardware.temps, false),
    ];

    item_view(node, content)
}

pub fn fan_view<'a>(node: &'a Node, hardware: &'a Hardware) -> Element<'a, AppMsg> {
    let content = vec![
        Text::new(format!("{} RPM", node.value.unwrap_or(0))).into(),
        pick_hardware(node, &hardware.fans, false),
    ];

    item_view(node, content)
}

pub fn custom_temp_view<'a>(node: &'a Node, nodes: &'a Nodes) -> Element<'a, AppMsg> {
    let NodeType::CustomTemp(custom_temp) = &node.node_type else {
        panic!()
    };

    let inputs = custom_temp
        .input
        .iter()
        .map(|i| {
            Row::new()
                .push(Text::new(i))
                // todo: icon
                .push(
                    Button::new(Text::new("remove"))
                    //.on_press(AppMsg::RemoveInput(node.id, Pick::new(i, id)))
                )
                .into()
        })
        .collect();
    
    //let pick_kind = PickList::new();
    let content = vec![
        
        pick_input(
            node,
            nodes,
            &Some("Choose Temp".into()),
            false,
            Box::new(AppMsg::AddInput),
        ),
        Column::with_children(inputs).into(),
    ];

    item_view(node, content)
}
