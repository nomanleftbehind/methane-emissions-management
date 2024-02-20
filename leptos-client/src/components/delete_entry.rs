use crate::models::mutations::manual_mutation::delete_entry::{
    DeleteEntryInput, DeleteEntryVariant, Variables,
};
use leptos::*;
use uuid::Uuid;

#[component]
pub fn delete_entry(
    id: Signal<Uuid>,
    row_num: Signal<usize>,
    col_num: Signal<usize>,
    delete_entry_variant: Signal<DeleteEntryVariant>,
    #[prop(into)] handle_delete_entry: Callback<Variables>,
) -> impl IntoView {
    view! {
        <div class="emitter-cell" class="center" style:grid-row=row_num style:grid-column=col_num>
            <button
                class="entry-button"
                on:click=move |_| {
                    handle_delete_entry
                        .call(Variables {
                            delete_entry_input: DeleteEntryInput {
                                id: id.get(),
                                delete_entry_variant: delete_entry_variant.get(),
                            },
                        })
                }
            >

                "🗑"
            </button>
        </div>
    }
}
