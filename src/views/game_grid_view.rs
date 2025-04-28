use sycamore::prelude::*;

use crate::models::game_grid::GameGrid;

#[component]
pub fn GameGridView() -> View {
    let game_grid = use_context::<GameGrid>();

    let tr_views: Vec<View> = game_grid
        .char_grid
        .iter()
        .map(|row| {
            let td_views: Vec<View> = row
                .map(|cell| {
                    let cell = cell.clone();
                    view! {
                        td {
                            input(bind:value=cell.alphabet,
                                    r#type="text",
                                    maxlength="1",
                                    size="1",
                                    disabled=true,
                                    class=cell.status.get().css_class())
                        }
                    }
                })
                .into();

            view! {
                tr {
                    (td_views)
                }
            }
        })
        .collect();

    view! {
        table {
            tbody {
                (tr_views)
            }
        }
    }
}
