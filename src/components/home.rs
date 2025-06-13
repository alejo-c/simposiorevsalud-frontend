use yew::prelude::*;

use crate::components::Navbar;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <header class="event-header">
                <Navbar />

                <div class="container">
                    <div class="header-content">
                        <figure class="banner-container">
                            <img src="img/banner.png" alt="Banner del Simposio" />
                        </figure>
                    </div>
                </div>
            </header>

            <main class="container">
                <h1>
                    {"I Simposio Internacional de Revistas Científicas de Ciencias de la Salud:"}
                </h1>
                <h2>
                    {"Experiencias editoriales de América Latina."}
                </h2>

                // Speakers and Flyer Section
                <article class="speakers-flyer-section">
                    <section class="speakers-info">
                        <h3>{"Ponentes Destacados"}</h3>
                        <div class="speakers-grid">
                            <div class="speaker-card">
                                <h4>{"Dra. Marilia Sá Carvalho"}</h4>
                                <p class="speaker-title">{"Editora en Jefe de Cadernos de Saúde Pública "}</p>
                                <p>{"Médica, mestre em Saúde Pública (Fiocruz), doutora em Engenharia Biomédica (UFRJ) com pós-doutorado em estatística na Lancaster University (Reino Unido). Pesquisadora titular da Fiocruz. Principais áreas de interesse: epidemiologia ambiental, pensamento sistêmico e modelagem de dados com estruturas complexas. ."}</p>
                            </div>
                            <div class="speaker-card">
                                <h4>{"Dr. Salvador Peiró"}</h4>
                                <p class="speaker-title">{"Director de Gaceta Sanitaria"}</p>
                                <p>{"M.D., Ph.D., Board Certified in Preventive Medicine and Public Health), is currently a senior researcher in the Health Services Research and Pharmacoepidemiology Unit at FISABIO Public Health, a biomedical research foundation dependent on the Valencia Government. He is also a collaborating researcher at the Centre for Research in Health Economics at Pompeu Fabra University (CRES-UPF) in Barcelona, an honorary collaborating professor in the Department of Pharmacology at the University of Málaga, and a member of the RICORS Network for Research in Chronicity, Primary Care, and Health Prevention and Promotion (RICAPPS)."}</p>
                            </div>
                            <div class="speaker-card">
                                <h4>{"Dra. Leticia Robles"}</h4>
                                <p class="speaker-title">{"Coordinadora de Indexación - SciELO"}</p>
                                <p>{"."}</p>
                            </div>
                            <div class="speaker-card">
                                <h4>{"Dr. Andrés Agudelo"}</h4>
                                <p class="speaker-title">{"Consultor Editorial - OPS/OMS"}</p>
                                <p>{"Editor en Jefe de la Revista de Odontología de la Universidad de Antioquia."}</p>
                            </div>
                        </div>
                    </section>

                    <section class="flyer-section">
                        <figure class="flyer-container">
                            <img src="img/flyer.png" alt="Flyer del Simposio" />
                        </figure>
                    </section>
                </article>

                // Event Information Section
                <article class="event-info">
                    <div class="info-grid">
                        <div class="location-info">
                            <h3>{"Ubicación del Evento"}</h3>
                            <div class="location-details">
                                <p>{"Universidad de Nariño"}</p>
                                <p>{"Auditorio Bloque Sur - Bloque 1B"}</p>
                                <p>{"Ciudad Universitaria Torobajo – Calle 18 No. 50-02"}</p>
                                <p>{"San Juan de Pasto - Nariño - Colombia"}</p>

                                <p class="mt-3"><strong>{"Modalidad:"}</strong></p>
                                <p>{"Presencial y Virtual (Híbrido)"}</p>
                                <p>{"Transmisión en vivo por plataforma Zoom"}</p>
                            </div>
                        </div>

                        <div class="date-info">
                            <h3>{"Fecha y Horarios"}</h3>
                            <div class="date-details">
                                <p><strong>{"Fecha: "}</strong>{"14 de Noviembre, 2025"}</p>
                                <p><strong>{"Hora de inicio: "}</strong>{"8:00 AM"}</p>
                                <p><strong>{"Hora de finalización: "}</strong>{"6:00 PM"}</p>

                                <div class="schedule-summary mt-3">
                                    <h4>{"Agenda Resumida:"}</h4>
                                    <ul>
                                        <li>{"7:00 AM: Registro de asistencia"}</li>
                                        <li>{"8:00 AM: Apertura"}</li>
                                        <li>{"8:30 AM: Salvador Peiró"}</li>
                                        <li>{"9:30 AM: Receso"}</li>
                                        <li>{"9:50 AM: Marilia Carvalho"}</li>
                                        <li>{"11:00 AM: Leticia Robles"}</li>
                                        <li>{"12:00 AM: Receso"}</li>
                                    </ul>
                                    <ul>
                                        <li>{"2:00: Andrés Agudelo"}</li>
                                        <li>{"3:00: Autores y Evaluador reconocidos"}</li>
                                        <li>{"4:00: Receso"}</li>
                                        <li>{"4:20: Foro"}</li>
                                        <li>{"5:20: Foro"}</li>
                                        <li>{"5:20: Foro"}</li>
                                    </ul>
                                </div>
                            </div>
                        </div>
                    </div>
                </article>
            </main>

            // Footer
            <footer class="event-footer">
                <div class="container">
                    <div class="footer-content">
                        <div class="contact-section">
                            <h4>{"Información de Contacto"}</h4>
                            <p><strong>{"Email:"}</strong> {"simposiorevsalud@udenar.edu.co"}</p>
                        </div>

                        <div class="organizers-section">
                            <h4>{"Organizadores"}</h4>
                            <p>{"Centro de Estudios en Salud de la Universidad de Nariño (CESUN)"}</p>
                            <p>{"Universidad de Nariño"}</p>
                        </div>
                    </div>

                    <div class="footer-bottom">
                        <p>{"2025 Revista Universidad y Salud. Todos los derechos reservados."}</p>
                    </div>
                </div>
            </footer>
        </>
    }
}
