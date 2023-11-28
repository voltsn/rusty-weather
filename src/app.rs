use leptos::*;
use leptos_router::*;
use crate::home::*;
use crate::about::*;

#[component]
pub fn App() -> impl IntoView{
    view! {
        <Router>
            <main>
              <Routes>
                <Route path="/" view=Home />
                <Route path="/about" view=About />
              </Routes>
            </main>
        </Router>

    }
}
