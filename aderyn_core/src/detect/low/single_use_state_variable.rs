use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{
    ElementaryTypeName, ElementaryTypeNameExpression, Expression, FunctionCall, Identifier, NodeID,
    TypeName,
};
use crate::ast::{FunctionCallKind, VariableDeclaration};

use crate::capture;
use crate::context::browser::{
    ExtractBinaryOperations, ExtractElementaryTypeNameExpressions, ExtractIdentifiers,
    ExtractIfStatements, GetImmediateChildren,
};
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};

use eyre::Result;

#[derive(Default)]
pub struct SingleUseStateVariableDetector {
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

/**
 * @author Stefania Pozzi
 * @description This detector searches if a state variable is set in a function.
 * The detector will search for ifs cointating the variable name that preceed the variable setting.
 * It the if is found, the value inside the variable is assumed to be checked before a new assignment.
 * This is not always the case, but it is a good indicator that the variable is not resetted.
 * 
 * Todo: improve efficiency
*/
impl IssueDetector for SingleUseStateVariableDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for state_variable in context.variable_declarations() {
            let mut state_variable_present_binary_operation = false;
            let mut state_variable_present_true_body = false;
            for function in context.function_definitions() {
                //1. variable must be present inside the binary operation
                for binary_operator in ExtractBinaryOperations::from(function).extracted {
                    for identifier in ExtractIdentifiers::from(&binary_operator).extracted {
                        if identifier.name == state_variable.name {
                            state_variable_present_binary_operation = true
                        }
                    }
                }
                //2. variable must  be present inside the if statement true body
                for if_statement in ExtractIfStatements::from(function).extracted {
                    for identifier in ExtractIdentifiers::from(&if_statement).extracted {
                        if identifier.name == state_variable.name {
                            state_variable_present_true_body = true
                        }
                    }
                }

                if state_variable_present_binary_operation && state_variable_present_true_body {
                    capture!(self, context, function); //add state var ;
                }
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Stack variable value is assigned and used only once")
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
        format!("{}", IssueDetectorNamePool::SingleUseStateVariable)
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
