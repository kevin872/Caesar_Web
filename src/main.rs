use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <style>
                {"
                body {
                    font-family: Comic sans-serif, sans-serif;
                    margin: 0;
                }
                
                header {
                    background-color: #333;
                    color: #fff;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: 1rem;
                }
                footer {
                    background-color: #333;
                    color: #fff;
                    text-align: center;
                    padding: 1rem;
                  }
                  nav ul {
                    list-style: none;
                    margin: 0;
                    padding: 0;
                    display: flex;
                  }
                  
                  nav li {
                    margin-right: 1rem;
                  }
                  
                  nav a {
                    color: #fff;
                    text-decoration: none;
                  }
                  
                  main {
                    max-width: 800px;
                    margin: 0 auto;
                    padding: 2rem;
                  }
                  
                  section {
                    margin-bottom: 2rem;
                  }
                  
                  h2 {
                    font-size: 2rem;
                    margin-top: 0;
                  }
                  
                  ul {
                    list-style: disc;
                  }
                  
                "}
            </style>
            <body>
        <header>
            <h1>{"Julius Caesar"}</h1>
            <nav>
            <ul>
                <li><a href="about.html">{"About the Play"}</a></li>
                <li><a href="characters.html">{"Characters"}</a></li>
                <li><a href="#themes">{"Themes"}</a></li>
                <li><a href="#analysis">{"Analysis"}</a></li>
            </ul>
            </nav>
        </header>
        <main>
            <section id="about">
            <h2>{"About the Play"}</h2>
            <li>{"William Shakespeare was a playwright who lived in England during the 16th and early 17th centuries, and is widely considered one of the greatest writers in the English language."}
        </li>
            <li>{"The play is set in ancient Rome and depicts the events leading up to the assassination of Julius Caesar by a group of conspirators, including the Roman senator Brutus. After Caesar's death, a power struggle ensues between the conspirators and Caesar's allies, led by Mark Antony and Octavius Caesar."}
        </li>
            <li>{"Julius Caesar was likely written by Shakespeare in 1599, and is one of his most famous and frequently-performed plays."}</li>
            
            <li>{"Some of the major themes explored in Julius Caesar include the dangers of ambition, the complexities of loyalty and betrayal, the corrupting influence of power, and the role of fate and free will in human affairs."}
        </li>
            <li>{"Julius Caesar is notable for its rich and complex characters, including the stoic and idealistic Brutus, the manipulative and cunning Cassius, the charismatic and vengeful Mark Antony, and the tragic and enigmatic figure of Julius Caesar himself."}
        </li>
            <li>{"The play has had a significant impact on Western culture, inspiring countless adaptations, interpretations, and references in literature, theater, film, and other media."}
        </li>
            


            </section>
            <section id="characters">
            <h2>{"Characters"}</h2>
            <ul>
                <li>{"Julius Caesar"}</li>
                <li>{"Brutus"}</li>
                <li>{"Cassius"}</li>
                <li>{"Mark Antony"}</li>
                <li>{"Calpurnia"}</li>
            </ul>
            </section>
            <section id="themes">
            <h2>{"Themes"}</h2>
            <ul>
                <li>{"Power and ambition"}</li>
                <li>{"Betrayal and loyalty"}</li>
                <li>{"Fate and free will"}</li>
            </ul>
            </section>
            <section id="analysis">
            <h2>{"Analysis"}</h2>
            <li>{"One of the key themes of Julius Caesar is the tension between fate and free will. Throughout the play, characters grapple with the question of whether they are in control of their own destiny, or whether their actions are predetermined by forces beyond their control."}
        </li>
            <li>{"Another important theme is the nature of power and authority, and the corrupting influence it can have on those who wield it. The play depicts a range of different characters who seek power for different reasons, and who are all ultimately consumed by it in one way or another."}
        </li>
            <li>{"The characters in Julius Caesar are all complex and multifaceted, with their own unique motivations and worldviews. Brutus, for example, is torn between his loyalty to his friend Caesar and his duty to the Roman Republic, while Mark Antony is driven by a desire for revenge and a thirst for power."}
        </li>
            <li>{"The language and rhetoric used in Julius Caesar are also a key aspect of the play's significance. Shakespeare employs a variety of literary techniques and devices, including imagery, metaphor, allusion, and symbolism, to convey the play's themes and messages."}
        </li>
            <li>{"There have been many different interpretations and adaptations of Julius Caesar over the years, each of which highlights different aspects of the play's meaning and significance. Some productions have focused on the political aspects of the play, while others have emphasized its psychological or philosophical dimensions."}
        </li>
            <li>{"Finally, the enduring popularity and relevance of Julius Caesar is a testament to its enduring power as a work of literature. Despite being hundreds of years old, the play continues to resonate with audiences today, and to offer insights into the human experience that are as relevant now as they were in Shakespeare's time."}
        </li>
            </section>
        </main>
        <footer>
            <p>{"2023 Julius Caesar Website"}</p>
        </footer>
        </body>
        </>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}