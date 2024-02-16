use leptos::{
    component, create_rw_signal, mount_to_body, provide_context, use_context, view, CollectView,
    IntoView, RwSignal, SignalGet, SignalUpdate,
};
use leptos_router::*;
use rand::*;

mod system;

use system::*;

fn main() {
    mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=Home/>
                <Route path="/play" view=Play/>
                <Route path="/credits" view=Credits/>
            </Routes>
        </Router>
    }
}

#[component]
fn Credits() -> impl IntoView {
    view! {
        <h1 class="center credits fade">
            "A project by " <a class="win" href="/"> "Rexiel" </a>
        </h1>
        <p class="wall unselectable">
        {

            let mut rng = thread_rng();
            let mut x_o = 0;

            (0..20000).map(|_| {
                if rng.gen_range(0..=50) == 0 {
                    x_o += 1;
                    if x_o % 2 == 0 { 'X' } else { 'O' }
                } else {'.'}
            }).collect::<String>()
        }
        </p >
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <a href="/play" class="big playbutton center fade">
            <a class="red"> "TIC" </a>
            <a> "TAC" </a>
            <a class="blue"> "TOE" </a>
        </a>
    }
}

#[component]
fn Play() -> impl IntoView {
    let grid = create_rw_signal(Grid::new());
    let won = move || grid.get().2;
    let draw = move || grid.get().get_turn() > 8 && !grid.get().2;

    provide_context(grid);

    view! {
        <h1>
            <a href="/" class="playbutton fade">
                <a class="red"> "TIC " </a>
                <a> "TAC " </a>
                <a class="blue"> "TOE " </a>
            </a>
        </h1>
        <div class="base fade">
            <div class="secondary">
                <button
                    class="reset"
                    on:click = move |_| grid.update(|g| g.reset())
                >
                    "reset"
                </button>
                <button
                    class="credits"
                    on:click = move |_| use_navigate()("/credits", Default::default())
                >
                    "credits"
                </button>
            </div>

            {
                (1..=3).map( |i|
                    view! {
                        <div class="row">
                        {
                            grid.get().row(i).iter()
                                .map(|c| view! { <DrawCell cell=c.clone()/> })
                                .collect_view()
                        }
                        </div>
                    }
                ).collect_view()
            }
        </div>
        <h1 class:win = move || won()>
        { move ||
            if won() {
                "YOU WIN"
            } else if draw() {
                "DRAW"
            } else {
                ""
            }
        }
        </h1>
    }
}

#[component]
fn DrawCell(cell: Cell) -> impl IntoView {
    let table = use_context::<RwSignal<Grid>>().unwrap();
    let odd = move || table.get().get_turn() % 2 == 0;

    view! {
        <button
            on:click = move |_| {
                if odd() {
                    table.update(|g| if  g.cell_stroke(cell.id, 'X') { g.update_turn() })
                } else {
                    table.update(|g| if  g.cell_stroke(cell.id, 'O') { g.update_turn() })
                }
            }
            class:red = move || cell.stroke.get() == 'X'
            class:blue = move || cell.stroke.get() == 'O'
        >
            { cell.stroke }
        </button>
    }
}
