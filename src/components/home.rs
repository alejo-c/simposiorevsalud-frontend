use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <h1>
                {"I Simposio Internacional de Revistas Cientificas de Ciencias de la Salud:"}
            </h1>
            <h2>
                {"Experiencias editoriales de América Latina."}
            </h2>

            <nav>
                <figure>
                    <img src="static/img/banner.png" alt="Banner" />
                </figure>
            </nav>

            <article>
                <section>
                </section>

                <section>
                    <figure>
                        <img src="static/img/flyer.png" alt="Flyer" />
                    </figure>
                </section>
            </article>

            <article>
            </article>

            <footer>
            </footer>
        </>
    }
}
