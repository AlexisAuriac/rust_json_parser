use ll_lexer::rule::{Rule, RuleTable};

use crate::symbol::GramSym;

pub fn get_rt() -> RuleTable<GramSym> {
    return RuleTable::new(
        vec![(GramSym::NtsValue, false)],
        GramSym::TsEos,
        vec![
            // Basic values
            Rule::new(GramSym::NtsValue, GramSym::TsNbr, vec![]),
            Rule::new(GramSym::NtsValue, GramSym::TsString, vec![]),
            Rule::new(GramSym::NtsValue, GramSym::TsBool, vec![]),
            // Array syntax
            Rule::new(
                GramSym::NtsValue,
                GramSym::TsLBracket,
                vec![
                    (GramSym::NtsArrayFirstVal, true),
                    (GramSym::TsRBracket, false),
                ],
            ),
            Rule::new(
                GramSym::NtsArrayFirstVal,
                GramSym::TsNbr,
                vec![(GramSym::NtsArrayNextVal, true)],
            ),
            Rule::new(
                GramSym::NtsArrayFirstVal,
                GramSym::TsString,
                vec![(GramSym::NtsArrayNextVal, true)],
            ),
            Rule::new(
                GramSym::NtsArrayFirstVal,
                GramSym::TsBool,
                vec![(GramSym::NtsArrayNextVal, true)],
            ),
            Rule::new(
                GramSym::NtsArrayNextVal,
                GramSym::TsComma,
                vec![(GramSym::NtsValue, false), (GramSym::NtsArrayNextVal, true)],
            ),
            // Object syntax
            Rule::new(
                GramSym::NtsValue,
                GramSym::TsLCurlyBraces,
                vec![
                    (GramSym::NtsFirstProperty, true),
                    (GramSym::TsRCurlyBraces, false),
                ],
            ),
            Rule::new(
                GramSym::NtsFirstProperty,
                GramSym::TsString,
                vec![
                    (GramSym::TsColon, false),
                    (GramSym::NtsValue, false),
                    (GramSym::NtsNextProperty, true),
                ],
            ),
            Rule::new(
                GramSym::NtsNextProperty,
                GramSym::TsComma,
                vec![
                    (GramSym::TsString, false),
                    (GramSym::TsColon, false),
                    (GramSym::NtsValue, false),
                    (GramSym::NtsNextProperty, true),
                ],
            ),
        ],
    );
}
