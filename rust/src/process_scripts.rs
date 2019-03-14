use brawllib_rs::script_ast::{EventAst, ForLoop, Iterations, IfStatement, Expression, UnaryExpression, BinaryExpression, ChangeAction};
use brawllib_rs::script_ast::variable_ast::{
    VariableAst,
    InternalConstantInt,
    LongtermAccessInt,
    LongtermAccessFloat,
    LongtermAccessBool,
    RandomAccessInt,
    RandomAccessFloat,
    RandomAccessBool,
};

use crate::brawl_data::{BrawlMod, BrawlFighter};

pub fn process_events(events: &[EventAst], common: bool, brawl_mod: &BrawlMod, fighter: &BrawlFighter) -> String {
    let script_lookup = if common {
        &fighter.script_lookup_common
    } else {
        &fighter.script_lookup
    };

    let mut result = String::from("<ol>");
    for event in events {
        match event {
            EventAst::Nop => { }
            EventAst::ChangeAction (ChangeAction { action, test }) => {
                if let Some(action) = fighter.fighter.actions.get(*action as usize) {
                    result.push_str(&format!("<li>ChangeAction {{ action: <a href='/{}/{}/actions/{}.html'>{}</a>, requirement: ({}) }}</li>",
                        brawl_mod.name, fighter.fighter.name, action.name, action.name, process_expression(test)));
                } else {
                    result.push_str(&format!("<li>{:?}</li>", event));
                    error!("Failed to lookup action for ChangeAction");
                }
            }
            EventAst::ChangeActionStatus { status_id, action, requirement, flip } => {
                if let Some(action) = fighter.fighter.actions.get(*action as usize) {
                    let test = if *flip {
                        Expression::Not(Box::new(Expression::Nullary(requirement.clone())))

                    } else {
                        Expression::Nullary(requirement.clone())
                    };

                    result.push_str(
                        &format!("<li>ChangeActionStatus {{ status_id: {}, action: <a href='/{}/{}/actions/{}.html'>{}</a>, requirement: ({}), }}</li>",
                        status_id, brawl_mod.name, fighter.fighter.name, action.name, action.name, process_expression(&test))
                    );
                } else {
                    result.push_str(&format!("<li>{:?}</li>", event));
                    error!("Failed to lookup action for ChangeAction");
                }
            }
            EventAst::ChangeSubaction (subaction) => {
                if let Some(subaction) = fighter.fighter.subactions.get(*subaction as usize) {
                    result.push_str(&format!("<li>ChangeSubaction(<a href='/{}/{}/subactions/{}.html'>{}</a>)</li>",
                        brawl_mod.name, fighter.fighter.name, subaction.name, subaction.name));
                } else {
                    result.push_str(&format!("<li>{:?}</li>", event));
                    error!("Failed to lookup action for ChangeSubaction");
                }
            }
            EventAst::ChangeSubactionRestartFrame (subaction) => {
                if let Some(subaction) = fighter.fighter.subactions.get(*subaction as usize) {
                    result.push_str(&format!("<li>ChangeSubactionRestartFrame(<a href='/{}/{}/subactions/{}.html'>{}</a>)</li>",
                        brawl_mod.name, fighter.fighter.name, subaction.name, subaction.name));
                } else {
                    result.push_str(&format!("<li>{:?}</li>", event));
                    error!("Failed to lookup action for ChangeSubactionRestartFrame");
                }
            }
            EventAst::ForLoop ( ForLoop { iterations, block } ) => {
                let iterations = match iterations {
                    Iterations::Finite (i) => i.to_string(),
                    Iterations::Infinite => "Infinite".to_string(),
                };
                result.push_str(&format!("<li>loop {} times: {}</li>", iterations, &process_events(&block.events, common, brawl_mod, fighter)));
            }
            EventAst::IfStatement ( IfStatement { test, then_branch, else_branch } ) => {
                result.push_str(&format!("<li>if ({}) {} </li>", process_expression(test), &process_events(&then_branch.events, common, brawl_mod, fighter)));

                if let Some(else_branch) = else_branch {
                    result.push_str("<li>else");
                    result.push_str(&process_events(&else_branch.events, common, brawl_mod, fighter));
                    result.push_str("</li>");
                }
            }
            EventAst::Goto (offset) => {
                if let Some(script_info) = script_lookup.get(&offset.offset) {
                    result.push_str(&format!("<li>Goto(<a href='{}'>{}</a>)</li>", script_info.address, script_info.name));
                } else {
                    if let Some(script) = fighter.fighter.scripts_section.iter().find(|x| x.callers.contains(&offset.origin)) {
                        result.push_str(&format!("<li>Goto(<a href='help'>External: {}</a>)</li>", script.name));
                    } else {
                        result.push_str(&format!("<li>Goto(Offset {{ offset: 0x{:x}, origin: 0x{:x} }})</li>", offset.offset, offset.origin));
                        error!("Failed to lookup script for goto destination");
                    }
                }
            }
            EventAst::Subroutine (offset) => {
                if let Some(script_info) = script_lookup.get(&offset.offset) {
                    result.push_str(&format!("<li>Subroutine(<a href='{}'>{}</a>)</li>", script_info.address, script_info.name));
                } else {
                    if let Some(script) = fighter.fighter.scripts_section.iter().find(|x| x.callers.contains(&offset.origin)) {
                        result.push_str(&format!("<li>Subroutine(<a href='/{}/{}/scripts_common/{}.html'>External: {}</a>)</li>", brawl_mod.name, fighter.fighter.name, script.name, script.name));
                    } else {
                        result.push_str(&format!("<li>Subroutine(Offset {{ offset: 0x{:x}, origin: 0x{:x} }})</li>", offset.offset, offset.origin));
                        error!("Failed to lookup script for subroutine destination");
                    }
                }
            }
            EventAst::CallEveryFrame { thread_id, offset } => {
                if let Some(script_info) = script_lookup.get(&offset.offset) {
                    result.push_str(&format!("<li>CallEveryFrame {{ thread_id: {}, script: <a href='{}'>{}</a> }}</li>", thread_id, script_info.address, script_info.name));
                } else {
                    if let Some(script) = fighter.fighter.scripts_section.iter().find(|x| x.callers.contains(&offset.origin)) {
                        result.push_str(&format!("<li>Subroutine(<a href='/{}/{}/scripts_common/{}.html'>External: {}</a>)</li>", brawl_mod.name, fighter.fighter.name, script.name, script.name));
                    } else {
                        result.push_str(&format!("<li>{:x?}</li>", event));
                        error!("Failed to lookup script for CallEveryFrame destination");
                    }
                }
            }
            EventAst::IntVariableSet { value, variable } => result.push_str(&format!("<li>IntVariableSet {{ variable: {}, value: {} }}</li>", process_expression(&Expression::Variable(variable.clone())), value)),
            EventAst::IntVariableAdd { value, variable } => result.push_str(&format!("<li>IntVariableAdd {{ variable: {}, value: {} }}</li>", process_expression(&Expression::Variable(variable.clone())), value)),
            EventAst::IntVariableSubtract { value, variable } => result.push_str(&format!("<li>IntVariableSubtract {{ variable: {}, value: {} }}</li>", process_expression(&Expression::Variable(variable.clone())), value)),
            EventAst::IntVariableIncrement { variable } => result.push_str(&format!("<li>IntVariableIncrement {{ variable: {} }}</li>", process_expression(&Expression::Variable(variable.clone())))),
            EventAst::IntVariableDecrement { variable } => result.push_str(&format!("<li>IntVariableDecrement {{ variable: {} }}</li>", process_expression(&Expression::Variable(variable.clone())))),
            EventAst::FloatVariableSet { value, variable } => result.push_str(&format!("<li>FloatVariableSet {{ variable: {}, value: {} }}</li>", process_expression(&Expression::Variable(variable.clone())), value)),
            EventAst::FloatVariableAdd { value, variable } => result.push_str(&format!("<li>FloatVariableAdd {{ variable: {}, value: {} }}</li>", process_expression(&Expression::Variable(variable.clone())), value)),
            EventAst::FloatVariableSubtract { value, variable } => result.push_str(&format!("<li>FloatVariableSubtract {{ variable: {}, value: {} }}</li>", process_expression(&Expression::Variable(variable.clone())), value)),
            EventAst::FloatVariableMultiply { value, variable } => result.push_str(&format!("<li>FloatVariableMultiply {{ variable: {}, value: {} }}</li>", process_expression(&Expression::Variable(variable.clone())), value)),
            EventAst::FloatVariableDivide { value, variable } => result.push_str(&format!("<li>FloatVariableDivide {{ variable: {}, value: {} }}</li>", process_expression(&Expression::Variable(variable.clone())), value)),
            EventAst::BoolVariableSetTrue { variable } => result.push_str(&format!("<li>BoolVariableSetTrue {{ variable: {} }}</li>", process_expression(&Expression::Variable(variable.clone())))),
            EventAst::BoolVariableSetFalse { variable } => result.push_str(&format!("<li>BoolVariableSetFalse {{ variable: {} }}</li>", process_expression(&Expression::Variable(variable.clone())))),
            EventAst::ItemThrow { unk1, unk2, unk3, unk4, unk5 } => result.push_str(&format!("<li>ItemThrow {{ unk1: {}, unk2: {}, unk3: {} unk4: {}, unk5: {} }}</li>",
                process_expression(&Expression::Variable(unk1.clone())),
                process_expression(&Expression::Variable(unk2.clone())),
                process_expression(&Expression::Variable(unk3.clone())),
                unk4.as_ref().map(|x| process_expression(&Expression::Variable(x.clone()))).unwrap_or("None".into()),
                unk5.as_ref().map(|x| process_expression(&Expression::Variable(x.clone()))).unwrap_or("None".into()),
            )),
            EventAst::ItemThrow2 { unk1, unk2, unk3 } => result.push_str(&format!("<li>ItemThrow2 {{ unk1: {}, unk2: {}, unk3: {}}}</li>", unk1, unk2, process_expression(&Expression::Variable(unk3.clone())))),
            EventAst::ApplyThrow (throw) => result.push_str(&format!("<li>ApplyThrow {{ unk0: {}, bone: {}, unk1: {} unk2: {}, unk3: {} }}</li>",
                throw.unk0,
                throw.bone,
                process_expression(&Expression::Variable(throw.unk1.clone())),
                process_expression(&Expression::Variable(throw.unk2.clone())),
                process_expression(&Expression::Variable(throw.unk3.clone())),
            )),
            EventAst::Unknown (event) => result.push_str(&format!("<li>UnknownEvent {{ namespace: 0x{:x}, code: 0x{:x}, unk1: 0x{:x}, arguments: {:?} }}</li>", event.namespace, event.code, event.unk1, event.arguments)),
            _ => result.push_str(&format!("<li>{:?}</li>", event)),
        }
    }
    result.push_str("</ol>");
    result
}

fn process_expression(expr: &Expression) -> String {
    match expr {
        Expression::Nullary (requirement) => format!("{:?}", requirement),
        Expression::Unary (UnaryExpression { requirement, value })
            => format!("{:?} {}", requirement, process_expression(value)),
        Expression::Binary (BinaryExpression { left, operator, right })
            => format!("{} {:?} {}", process_expression(left), operator, process_expression(right)),
        Expression::Not (expr) => format!("not({})", process_expression(expr)),
        Expression::Variable (variable) =>
            // TODO: This handler needs to be also called by stuff like IntVariableSet
            match variable {
                VariableAst::InternalConstantInt (InternalConstantInt::Address (address)) => format!("InternalConstantInt (0x{:x})", address),
                VariableAst::LongtermAccessInt   (LongtermAccessInt::Address   (address)) => format!("LongtermAccessInt (0x{:x})", address),
                VariableAst::LongtermAccessFloat (LongtermAccessFloat::Address (address)) => format!("LongtermAccessFloat (0x{:x})", address),
                VariableAst::LongtermAccessBool  (LongtermAccessBool::Address  (address)) => format!("LongtermAccessBool (0x{:x})", address),
                VariableAst::RandomAccessInt     (RandomAccessInt::Address     (address)) => format!("RandomAccessInt (0x{:x})", address),
                VariableAst::RandomAccessFloat   (RandomAccessFloat::Address   (address)) => format!("RandomAccessFloat (0x{:x})", address),
                VariableAst::RandomAccessBool    (RandomAccessBool::Address    (address)) => format!("RandomAccessBool (0x{:x})", address),
                VariableAst::Unknown { memory_type, data_type, address } =>
                    format!("Unknown {{ memory_type: {:?}, data_type: {:?}, address: 0x{:x} }}", memory_type, data_type, address),
                _ => format!("{:?}", variable),
            }
        Expression::Value (value) => format!("value({})", value),
        Expression::Scalar (scalar) => format!("scalar({})", scalar),
    }
}
