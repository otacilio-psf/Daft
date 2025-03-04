use std::sync::Arc;

use common_error::DaftResult;
use daft_dsl::ExprRef;
use daft_micropartition::MicroPartition;

use super::intermediate_op::IntermediateOperator;

#[derive(Clone)]
pub struct FilterOperator {
    predicate: ExprRef,
}

impl FilterOperator {
    pub fn new(predicate: ExprRef) -> Self {
        Self { predicate }
    }
}

impl IntermediateOperator for FilterOperator {
    fn execute(&self, input: &Arc<MicroPartition>) -> DaftResult<Arc<MicroPartition>> {
        log::debug!("FilterOperator::execute");
        let out = input.filter(&[self.predicate.clone()])?;
        Ok(Arc::new(out))
    }

    fn name(&self) -> String {
        "FilterOperator".to_string()
    }
}
