use crate::command::prelude::*;

#[derive(Debug)]
pub struct FlowOutputCommand {
    rename: Name,
}

pub const FLOW_OUTPUT: &str = "flow_output";

impl FlowOutputCommand {
    fn new(data: &NodeData) -> Self {
        let form = &data.targets_form.form_data;

        let rename = form
            .get("label")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_owned();

        Self { rename }
    }
}

#[async_trait]
impl CommandTrait for FlowOutputCommand {
    fn name(&self) -> Name {
        FLOW_OUTPUT.into()
    }

    fn inputs(&self) -> Vec<Input> {
        [Input {
            name: "".into(),
            type_bounds: [ValueType::Free].to_vec(),
            required: true,
            passthrough: false,
        }]
        .to_vec()
    }

    fn outputs(&self) -> Vec<Output> {
        [Output {
            name: self.rename.clone(),
            r#type: ValueType::Free,
            optional: false,
        }]
        .to_vec()
    }

    async fn run(&self, _ctx: Context, inputs: ValueSet) -> Result<ValueSet, CommandError> {
        Ok(match inputs.into_values().next() {
            Some(value) => ValueSet::from([(self.rename.clone(), value)]),
            None => ValueSet::new(),
        })
    }
}

inventory::submit!(CommandDescription::new(FLOW_OUTPUT, |data: &NodeData| {
    Ok(Box::new(FlowOutputCommand::new(data)))
}));
