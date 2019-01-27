use brawllib_rs::script_ast::{EventAst, IfStatement, Expression, UnaryExpression, BinaryExpression};

use crate::brawl_data::{BrawlMod, BrawlFighter};

pub fn process_events(events: &[EventAst], brawl_mod: &BrawlMod, fighter: &BrawlFighter) -> String {
    let script_lookup = &fighter.script_lookup;
    let mut result = String::from("<ol>");
    for event in events {
        match event {
            EventAst::Nop => { }
            EventAst::ChangeAction { action, test } => {
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
            EventAst::IfStatement ( IfStatement { test, then_branch, else_branch } ) => {
                result.push_str(&format!("<li>if ({}) {} </li>", process_expression(test), &process_events(&then_branch.events, brawl_mod, fighter)));

                if let Some(else_branch) = else_branch {
                    result.push_str("<li>else");
                    result.push_str(&process_events(&else_branch.events, brawl_mod, fighter));
                    result.push_str("</li>");
                }
            }
            EventAst::Goto (goto) => {
                if let Some(script_info) = script_lookup.get(&(*goto as u32)) {
                    result.push_str(&format!("<li>Goto(<a href='{}'>{}</a>)</li>", script_info.address, script_info.name));
                } else {
                    result.push_str(&format!("<li>{:?}</li>", event));
                    error!("Failed to lookup script for goto destination");
                }
            }
            EventAst::Subroutine (goto) => {
                if let Some(script_info) = script_lookup.get(&(*goto as u32)) {
                    result.push_str(&format!("<li>Subroutine(<a href='{}'>{}</a>)</li>", script_info.address, script_info.name));
                } else {
                    result.push_str(&format!("<li>{:?}</li>", event));
                    error!("Failed to lookup script for goto destination");
                }
            }
            EventAst::Unknown (event) => result.push_str(&format!("<li>Unknown event {:x?}</li>", event)),
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
        Expression::Variable (variable) => format!("variable(0x{:x})", variable),
        Expression::Value (value) => format!("value({})", value),
        Expression::Scalar (scalar) => format!("scalar({})", scalar),
    }
}
