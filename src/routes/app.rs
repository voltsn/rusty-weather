use leptos::*;
use leptos_router::*;
use crate::routes::home::Home;
use crate::routes::about::About;

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
