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
    ExtractIfStatements, ExtractVariableDeclarationStatements, GetImmediateChildren,
};
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};

use cyfrin_foundry_compilers::artifacts::contract;
use eyre::Result;

#[derive(Default)]
pub struct SingleUseStateVariableDetector {
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

/**
 * @author Stefania Pozzi
 * @description This detector searches if a state variable is used more than once inside a function.
 * For each function, it will first consider all its variable declarations
 * and then check if an identifier with the same name is present only once.
 *
 * Todo: improve efficiency
*/
impl IssueDetector for SingleUseStateVariableDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for function in context.function_definitions() {
            for variable_declaration_statement in
                ExtractVariableDeclarationStatements::from(function).extracted
            {
                let mut stack_variable_counter = 0;
                // 1. find the declared stack variable
                if let Some(variable_declaration) =
                    variable_declaration_statement.declarations.get(0)
                {
                    if let Some(variable_declaration) = variable_declaration {
                        let declared_var_name = &variable_declaration.name;

                        // 2. find all the identifiers in the function and increment counter
                        for identifier in ExtractIdentifiers::from(function).extracted {
                            if identifier.name.as_str() == declared_var_name {
                                if stack_variable_counter > 1 {
                                    // if the stack variable is used more than once, is a valid usage
                                    return Ok(!self.found_instances.is_empty());
                                } else {
                                    stack_variable_counter += 1;
                                }
                            }
                        }
                        if stack_variable_counter == 1 {
                            capture!(self, context, variable_declaration_statement)
                        }
                    }
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
