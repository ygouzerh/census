use yew::prelude::*;
use serde::Deserialize;
use reqwasm::http::Request;
use commons::{Population, Districts};

#[derive(Properties, PartialEq, Debug, Deserialize)]
struct CensusProps {
    census: Vec<commons::Population>
}

#[derive(Properties, PartialEq, Debug, Deserialize)]
struct DistrictsProps {
    districts: Vec<Districts>
}

#[function_component(PopulationList)]
fn population_list(CensusProps { census }: &CensusProps) -> Html {
    census.iter().map(|population| html! {
        <p class="text-indigo-600 sm:my-4 sm:text-md bg-white-200 rounded">
            <span class="underline">{ format!("{}", population.district) }</span>
            {" - "}
            <span class="underline">{ format!("{}", population.age) }</span>
            {" : "}
            <span class="font-bold">{ format!("{}", population.count) }</span>
        </p>
    }).collect()
}

#[function_component(DistrictsList)]
fn districts_list(DistrictsProps { districts }: &DistrictsProps) -> Html {
    districts.iter().map(|district| html! {
        <p class="text-indigo-600 sm:my-4 sm:text-md bg-white-200 rounded">
            <span class="underline">{ format!("{}", district) }</span>
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
                log::info!("{:?}", fetched_census);
                census.set(fetched_census);
            });
            || ()
        }, ());
    }
    
    let districts : UseStateHandle<Vec<Districts>> = use_state(|| vec![]);
    {
        let districts = districts.clone();
        use_effect_with_deps(move |_| {
            let districts = districts.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_districts : Vec<Districts> = Request::get("/api/districts")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                districts.set(fetched_districts);
                log::info!("{:?}", districts);
            });
            || ()
        }, ());
    }

    html! {
        <>
            <h1 class="text-2xl font-bold text-indigo-600 my-3 mx-2 mx-auto md:max-w-md max-w-sm text-center break-normal">{ "Population Census - Hong Kong" }</h1>
            <div class="bg-emerald-200 px-4 py-2 mx-auto md:max-w-md max-w-sm rounded">
                <DistrictsList districts = {(*districts).clone()} />
            </div>
            <div class="bg-emerald-200 px-4 py-2 mx-auto md:max-w-md max-w-sm rounded">
                <PopulationList census = {(*census).clone()} />
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}