//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  23oc24                                             //
//---------------------------------------------------------------//



use serde::Deserialize;
use yew::prelude::*;
use yew::platform::spawn_local;
use gloo_net::http::Request;



#[derive(Clone, PartialEq, Deserialize)]
struct Repo {
    id: u64,
    name: String,
    full_name: String,
    description: Option<String>,
    html_url: String,
    //stargazers_count: u32,
    language: Option<String>,
    //forks_count: u32,
}


#[derive(Properties, PartialEq)]
struct RepoCardProps {
    repo: Repo,
}


#[function_component(RepoCard)]
fn repo_card(props: &RepoCardProps) -> Html {
    let repo = &props.repo;
    
    html! {
        <div class={
            match repo.language.as_deref().unwrap_or("Not specified") {
                "Haskell" => "card mb-4 haskell-card",
                "Rust" => "card mb-4 rust-card",
                "Lua" => "card mb-4 lua-card",
                _ => "card mb-4 default-card"
            }
            
        }>
            <div class={"card-body"}>
                <h5 class="card-title">
                    <a href={repo.html_url.clone()} target="_blank" class="project-link text-primary">{&repo.name}</a>
                </h5>
                <p class="card-text">{repo.description.clone().unwrap_or_else(|| "No description".to_string())}</p>
                <div class="d-flex justify-content-between language-text">
                    <span>{"Language: "}{repo.language.clone().unwrap_or_else(|| "Not specified".to_string())}</span>
                   // <span>{"Stars: "}{repo.stargazers_count}</span>
                    //<span>{"Forks: "}{repo.forks_count}</span>
                </div>
            </div>
        </div>
    }
}


#[function_component(Projects)]
pub fn projects() -> Html {
    let repos = use_state(Vec::new);
    let loading = use_state(|| true);
    let error = use_state(|| None);

    {
        let repos = repos.clone();
        let loading = loading.clone();
        let error = error.clone();
        use_effect_with((), move |_| {
            fetch_repos(repos, loading, error);
            || ()
        });
    }

    html! {
        <div class="projects">
            <div class="container py-4 cards">
                if *loading {
                    <div class="d-flex justify-content-center">
                        <div class="spinner-border text-white" role="status">
                            <span class="visually-hidden">{"Loading..."}</span>
                        </div>
                    </div>
                } else if let Some(err) = error.as_ref() {
                    <div class="alert alert-danger" role="alert">
                        {"Error: "}{err}
                    </div>
                } else {
                    <div class="row row-cols-1 row-cols-lg-3 g-4">
                        {repos.iter().map(|repo| html! {
                            <div class="col" key={repo.id.to_string()}>
                                <RepoCard repo={repo.clone()} />
                            </div>
                        }).collect::<Html>()}
                    </div>
                }
            </div>
        </div>
    }
}


fn fetch_repos(repos: UseStateHandle<Vec<Repo>>, loading: UseStateHandle<bool>, error: UseStateHandle<Option<String>>) {
    spawn_local(async move {
        loading.set(true);
        error.set(None);

        match Request::get("https://api.github.com/users/sfmolina/repos")
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
        {
            Ok(response) => {
                match response.json::<Vec<Repo>>().await {
                    Ok(fetched_repos) => {
                        repos.set(fetched_repos);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to parse response: {}", e)));
                    }
                }
            }
            Err(e) => {
                error.set(Some(format!("Failed to fetch repos: {}", e)));
            }
        }

        loading.set(false);
    });
}
