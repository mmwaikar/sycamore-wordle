use sycamore::prelude::*;

use crate::view_models::guess_grid_vm::GuessGridVM;

#[component]
pub fn GameGridView() -> View {
    let guess_grid_vm = use_context::<GuessGridVM>();

    let tr_views: Vec<View> = guess_grid_vm
        .guess_alphabets
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
