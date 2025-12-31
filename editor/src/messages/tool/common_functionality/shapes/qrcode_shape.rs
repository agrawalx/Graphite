use super::shape_utility::ShapeToolModifierKey;
use super::*;
use crate::messages::portfolio::document::graph_operation::utility_types::TransformIn;
use crate::messages::portfolio::document::node_graph::document_node_definitions::resolve_document_node_type;
use crate::messages::portfolio::document::utility_types::document_metadata::LayerNodeIdentifier;
use crate::messages::portfolio::document::utility_types::network_interface::NodeTemplate;
use crate::messages::tool::tool_messages::tool_prelude::*;
use glam::{DAffine2, DVec2};
use graph_craft::document::NodeInput;
use graph_craft::document::value::TaggedValue;
use std::collections::VecDeque;

#[derive(Default)]
pub struct QrCode;

impl QrCode {
	pub fn create_node(text: String, ecc: String) -> NodeTemplate {
		let node_type = resolve_document_node_type("QR Code").expect("QR Code node can't be found");
		node_type.node_template_input_override([
			None,
			Some(NodeInput::value(TaggedValue::String(text), false)), // Input 1: Text
			Some(NodeInput::value(TaggedValue::String(ecc), false)),  //Input 2: Error Correction
			Some(NodeInput::value(TaggedValue::F64(100.), false)),    //Input 3: Width
		])
	}

	pub fn update_shape(
		document: &DocumentMessageHandler,
		ipp: &InputPreprocessorMessageHandler,
		viewport: &ViewportMessageHandler,
		layer: LayerNodeIdentifier,
		shape_tool_data: &mut ShapeToolData,
		modifier: ShapeToolModifierKey,
		responses: &mut VecDeque<Message>,
	) {
		let [center, lock_ratio, _] = modifier;

		if let Some([start, end]) = shape_tool_data.data.calculate_points(document, ipp, viewport, center, lock_ratio) {
			// Calculate the size of the drawn box 
			// taking max element to ensure qr code is a square
			let size = DVec2::new(start.x - end.x, start.y - end.y).abs();
			let width = size.max_element();

			// Update the transform of the layer to match the drawn box.
			responses.add(GraphOperationMessage::TransformSet {
				layer,
				transform: DAffine2::from_scale_angle_translation(DVec2::ONE, 0., start.midpoint(end)),
				transform_in: TransformIn::Viewport,
				skip_rerender: false,
			});

			// Update the width input of the node
			responses.add(NodeGraphMessage::SetInputValue {
				node_id: layer.to_node(),
				input_index: 3, // Width
				value: TaggedValue::F64(width),
			});
		}
	}
}
