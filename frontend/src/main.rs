use yew::prelude::*;
use serde::Deserialize;
use reqwasm::http::Request;
use commons;

#[derive(Properties, PartialEq, Debug, Deserialize)]
struct CensusProps {
    census: Vec<commons::Population>
}

#[function_component(PopulationList)]
fn population_list(CensusProps { census }: &CensusProps) -> Html {
    census.iter().map(|population| html! {
        <p class="text-indigo-600 sm:my-4 sm:text-md">
            <span class="underline">{ format!("{}", population.age) }</span>
            {" : "}
            <span class="font-bold">{ format!("{}", population.count) }</span>
        </p>
    }).collect()
}

#[function_component(App)]
fn app() -> Html {
    // usage
    wasm_logger::init(wasm_logger::Config::default());
    let census : UseStateHandle<Vec<commons::Population>> = use_state(|| vec![]);
    {
        let census = census.clone();
        use_effect_with_deps(move |_| {
            let census = census.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_census : Vec<commons::Population> = Request::get("/api/census")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                census.set(fetched_census);
                log::info!("{:?}", census);
            });
            || ()
        }, ());
    }

    // let census = vec![
    //     Population {
    //         age: String::from("15 - 24"),
    //         count: 783000
    //     },
    //     Population {
    //         age: String::from("65 and over"),
    //         count: 1102500
    //     }
    // ];


    html! {
        <>
            <h1 class="text-2xl font-bold text-indigo-600 my-3 mx-2 mx-auto md:max-w-md max-w-sm text-center break-normal">{ "Population Census - Hong Kong" }</h1>
            <div class="bg-emerald-200 px-4 py-2 mx-auto md:max-w-md max-w-sm rounded">
                <PopulationList census = {(*census).clone()} />
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}