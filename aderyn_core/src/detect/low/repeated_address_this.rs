use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::FunctionCallKind;
use crate::ast::{
    ElementaryTypeName, ElementaryTypeNameExpression, Expression, FunctionCall, Identifier, NodeID,
    TypeName,
};

use crate::capture;
use crate::context::browser::{
    ExtractElementaryTypeNameExpressions, ExtractIdentifiers, GetImmediateChildren,
};
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};

use eyre::Result;

#[derive(Default)]
pub struct RepeatedAddressThisDetector {
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

/**
 * @author Stefania Pozzi
 * @description This detector finds all address(this) calls in a contract.
 * If it has been found more than 2 times, report it.
*/
impl IssueDetector for RepeatedAddressThisDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let mut repeated_function_calls: Vec<&FunctionCall> = vec![];
        for call in context.function_calls() {
            if call.kind == FunctionCallKind::TypeConversion {
                let mut address_found = false;
                //search 'this'
                let identifiers: Vec<Identifier> = ExtractIdentifiers::from(call).extracted;
                let this_found =  identifiers.len() == 1 && identifiers.get(0).unwrap().name == "this";
                //search 'address'
                let elementary_type_name_expression: Vec<ElementaryTypeNameExpression> =
                    ExtractElementaryTypeNameExpressions::from(call).extracted;
                if elementary_type_name_expression.len() == 1 {
                    let elementary_type_name: &TypeName =
                        &elementary_type_name_expression.get(0).unwrap().type_name;
                    if let TypeName::ElementaryTypeName(ElementaryTypeName { name, .. }) =
                        elementary_type_name
                    {
                        if name == "address" {
                            address_found = true;
                        }
                    }
                }
                if address_found && this_found {
                    repeated_function_calls.push(call);
                }
            }
        }
        if repeated_function_calls.len() > 1 {
            for call in repeated_function_calls {
                capture!(self, context, call);
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Multiple address(this) calls inside the contract <contract-name> can cause uncecessary gas consumption.")
    }

    fn description(&self) -> String {
        String::from("")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::AddressThis)
    }
}

#[cfg(test)]
mod template_detector_tests {
    // use crate::detect::{detector::IssueDetector, low::template_detector::TemplateDetector};

    #[test]
    fn test_template_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/ArbitraryTransferFrom.sol",
        );

        // let mut detector = TemplateDetector::default();
        // let found = detector.detect(&context).unwrap();
        // // assert that the detector found an issue
        // assert!(found);
        // // assert that the detector found the correct number of instances
        // assert_eq!(detector.instances().len(), 1);
        // // assert the severity is low
        // assert_eq!(
        //     detector.severity(),
        //     crate::detect::detector::IssueSeverity::Low
        // );
        // // assert the title is correct
        // assert_eq!(detector.title(), String::from("Low Issue Title"));
        // // assert the description is correct
        // assert_eq!(
        //     detector.description(),
        //     String::from("Description of the low issue.")
        // );
    }
}
