use penrose::{core::layout::LayoutStack, stack, builtin::layout::{transformers::{ReserveTop, Gaps}, MainAndStack, Monocle}};

use crate::constants::*;

pub fn layouts_config() -> LayoutStack {
    stack!(
        MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP),
        MainAndStack::side_mirrored(MAX_MAIN, RATIO, RATIO_STEP),
        MainAndStack::bottom(MAX_MAIN, RATIO, RATIO_STEP),
        Monocle::boxed()
    )
    .map(|layout| ReserveTop::wrap(Gaps::wrap(layout, OUTER_PX, INNER_PX), BAR_HEIGHT_PX))
}