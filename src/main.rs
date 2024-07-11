#![allow(non_snake_case)]
use chrono::{DateTime, Duration, NaiveDate, Utc};
use dioxus::prelude::*;
use dioxus_core::ElementId;
use futures::future::join_all;
use log::error;
use serde::{Deserialize, Serialize};
use thiserror::Error;

static table_data: GlobalSignal<Vec<Vec<(String, i64, i64)>>> = Signal::global(|| vec![]);
pub static BASE_API_URL: &str = "http://192.168.0.67:8000/";
const _: &str = manganis::mg!(file("./global.css"));
use std::{clone, sync::{Arc, RwLock}};

#[derive(Error, Debug)]
pub enum MyError {
    #[error("HTTP request error")]
    ReqwestError(#[from] reqwest::Error),
    
    #[error("JSON parse error")]
    SerdeJsonError(#[from] serde_json::Error),
}

#[derive(Clone, Debug)]
enum PreviewState {
    Unset,
    Loading,
    Loaded(Vec<Todo>, i64, i64, bool),
    Error(String)
}

#[derive(Clone, Debug)]
enum UpdateState {
   Run,
   Done,
   Fail
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Module {
    pub modulename: String,
    pub examdate: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub done: bool,
    pub id: i64
}

#[derive(strum_macros::Display)]
pub enum Semester {
    Sose24,
    Wise24,
    Sose25,
    Wise25,
    Sose26,
    Wise26,
    Sose27,
    Wise27,
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    launch(App);
}

pub fn App() -> Element {
    use_context_provider(|| Signal::new(PreviewState::Unset));
    use_context_provider(|| Signal::new(UpdateState::Done));
    rsx! {
        div { display: "flex", flex_direction: "row", width: "100%",
            div { Calendar {} },
            div { width: "100%", Panel {} } //wip
            //div { width: "50%", Preview {} }
        }
    }
}
fn Panel() -> Element {
    log::info!("panel loading");
    let preview_state = consume_context::<Signal<PreviewState>>();
    let mut title = use_signal( || "bob".to_string());
    let mut number = use_signal( || 1);
    match preview_state() {
        PreviewState::Unset => rsx! {"click on cell to view panel"},
        PreviewState::Error(err) => rsx! {{err}},
        PreviewState::Loading => rsx! {"Loading..."},
        PreviewState::Loaded(mut todo_vector,x,y, this_is_an_update) => {
            let mut update_state = consume_context::<Signal<UpdateState>>();            
            todo_vector.sort_by_key(|d| d.id);
            //name.set(todo_vector);
            let veganstringcheese = "yummy".to_string();
            log::info!("preview_loaded");
            log::info!("x: {}",x);
            log::info!("y: {}",y);

            let data = table_data.read();
            log::info!("data dimensions: {} x {}", data.len(), data[0].len());
            //table_data.read()[y as usize][x as usize].2
            let how_many_todos_todo=todo_vector.clone().into_iter().filter(|todo| todo.done==false).collect::<Vec<Todo>>().len();
            let header = format!("{} todos left for {} days => {} todos/day",how_many_todos_todo,data[1][x as usize +1].2, todo_vector.len() as f32 /data[1][x as usize +1].2 as f32);

           // let cloned_todos=todo_vector.clone();
            //let value = cloned_todos.clone();
            rsx! {
                div{
                    class: "ChecklistContainer",
                    div{
                        id: "progressbar"
                    }
                    div{
                        h2{
                            "{header}"
                        },
                        input{
                            value:"{title}",
                            oninput: move |evt| title.set(evt.value()),
                        },
                        button{
                            onclick: move |_| {
                                update_checklist(preview_state,update_state,x,y,0 as i64,4,title())
                            },
                            "create"
                        },
                        input{
                            value:"{number}",
                            oninput: move |evt| number.set(evt.value().parse::<i32>().unwrap_or(1)),
                        }
                        button{
                            onclick: move |_| {
                                update_checklist(preview_state,update_state,x,number() as i64,0 as i64,5,title())
                            },
                            "create x times"
                        }
                        button{
                            onclick: move |_| {
                               // squeeze_calendar(todo_vector.clone(),x)
                            },
                            "squeeze todos"
                        }


                        
                        
                    }
                        ul{
                            id: "Checklist",
                            for (index, todo) in todo_vector.clone().iter().enumerate(){
                                li{
                                    
                                    class: "pseudo_li {x}",
                                    class:"{x}",
                                    id: "{index}",
                                    class: if let UpdateState::Run=update_state() {"loading"} ,
                                    class: if todo.done { "checked {x}" },
                                    key: "{index}", 
                                    //class: if todo.done { "checked" } else { "" }, 
                                    {todo.title.clone()}
                                    div{
                                   //     right: "0px",
                                        button{
                                            onclick: move |_| {
                                                update_checklist(preview_state,update_state,x,y,index as i64,2,"veganstringcheese".to_string())
                                            },
                                            class: "move_up_button",
                                            display: "block",
                                            "Up"
                                        }
                                        button{
                                            onclick: move |_| {
                                                update_checklist(preview_state,update_state,x,y,index as i64,3,"veganstringcheese".to_string())
                                            },
                                            class: "move_down_button",
                                            display: "block",
                                            "Dn"
                                        }
                                    },
                                    {
                                        let todo_id = todo.id;
                                        rsx!{
                                            div{
                                                onclick: move|_| {
                                                
                                                update_checklist(preview_state,update_state,x,y,todo_id ,1,"veganstringcheese".to_string())
                                                },
                                                left: "0px",
                                                width: "20px",
                                                height: "20px",
                                                background: "black"
                                            }
                                        }
                                        
                                    }
                                        
                                   
                                }
                            }
                        }
                }
                    
            }
        }
    }
}


async fn squeeze_calendar(todos:Vec<Todo>,index:i64){
    
    // Temporary vector to store updates
    let mut updates = Vec::new();

    // Collect updates
    for (count_out, row) in table_data.read().iter().clone().enumerate() {
        for (count_in, cell) in row.iter().enumerate() {
            if cell.1 == index {
                if count_out < todos.len() {
                    updates.push((count_out, count_in, todos[count_out].title.clone(), cell.1, cell.2));
                    log::info!("{}",format!("{},{},{}",count_out,count_in,todos[count_out].title.clone()));
                }
            }
        }
    }

    // Apply updates
    

    let todos_test: Vec<(String, i64, i64)> = 
        vec![("todo1.clone()".to_string(), 1,2 ), ("todo1.clone()".to_string(), 1,2 )]; // First row of todos
         // Second row of todos
    

    async move {
        //table_data.write().push(todos_test);
    }.await;
}


pub async fn get_todos(module_id: i64) -> Result<Vec<Todo>, MyError> {
    let url = format!("{}todos/{}.json", BASE_API_URL, module_id);
    let response = reqwest::get(&url).await?;
    
    let body = response.text().await?;
    log::info!("Response Body: {}", body);
    //log::info!("Response: {:?}", response);
    let todo_futures: Vec<Todo> = serde_json::from_str(&body)?;
    log::info!("vec: {:?}", todo_futures);
    Ok(todo_futures)
}


fn Calendar() -> Element {

    //let mut todos_fix = consume_context::<Signal<Vec<Vec<(String, i64, i64)>>>>();

    let modules = use_resource(move || get_modules(Semester::Sose24));

    match &*modules.read_unchecked() {
        Some(Ok(list)) => {
            let today = Utc::now().naive_utc().date();
            // Create a 2-dimensional array with all the table fields
            //let mut table_data: Vec<Vec<(String, i64, i64)>> = vec![];
            // Create the headers row
            let mut headers: Vec<(String, i64, i64)> = vec![("".to_string(), -1, -1)];
            headers.extend(
                list.iter()
                    .enumerate()
                    .map(|module| (module.1.modulename.clone(), module.0 as i64, -1)),
            );
            table_data.write().push(headers);
            // Calculate the days until the exam date for each module
            let mut reached_exam = false;
            for day_offset in 0.. {
                if reached_exam {
                    break;
                }
                let mut row: Vec<(String, i64, i64)> = vec![];
                let current_date = today + Duration::days(day_offset);
                row.push((current_date.format("%m-%d").to_string(), -1, day_offset));
                reached_exam = true; // Assume we will reach the exam date in this iteration

                for module in list.iter().enumerate() {
                    match NaiveDate::parse_from_str(&module.1.examdate, "%d-%m-%Y") {
                        Ok(exam_date) => {
                            let days_remaining = (exam_date - current_date).num_days();
                            if days_remaining >= 0 {
                                reached_exam = false;
                                //let stringi: String = format!("border: \"1px solid\",{}",days_remaining.to_string());
                                row.push((
                                    days_remaining.to_string(),
                                    module.0 as i64,
                                    days_remaining as i64,
                                ));
                                //row.push(stringi);
                            } else {
                                row.push(("".to_string(), 0, 0));
                            }
                        }
                        Err(e) => {
                            error!(
                                "Error parsing date for module {}: {}",
                                module.1.modulename, e
                            );
                            row.push(("Invalid date".to_string(), 0, 0));
                        }
                    }
                }

                table_data.write().push(row);
                //todos_fix.write().push(row);
            }
            //let module_overview = use_signal(|| None);
            let preview_state = consume_context::<Signal<PreviewState>>();
            rsx! {
                table {
                    style:"empty-cells: hide; table-layout: fixed; ",
                    border: "3px solid",
                    id:"calendar",
                    //empty-cells: "hide",
                    // Display the table using nested loops
                    for row in table_data.read().iter() {
                        //for row in todos_fix.read().iter() {
                        tr {
                            for cell in row.clone() {
                                td {
                                    onclick: move |_event| { cell_click(cell.1, cell.2, preview_state,false) },
                                    width:"10px",
                                    height:"10px",
                                    class: "{cell.2}",
                                    id: "{cell.1}",
                                    border: if cell.1<0 {"0px"} else {"1px solid"},

                                    "{cell.0}",

                                }
                            }
                        }
                    }
                }
            }
        }
        Some(Err(err)) => rsx! {"An error occurred while fetching modules {err}"},
        None => rsx! {"Loading items"},
    }
}

async fn cell_click(x: i64, y: i64, mut preview_state: Signal<PreviewState>,we_abuse_this_function_to_fetch_the_data_we_just_updated:bool) {
    *preview_state.write() = PreviewState::Loading;
    log::info!("{}",format!("{} :blb: {}",x.to_string(), y.to_string()));
    log::info!("Some info");
    let todos = get_todos(x);


    match todos.await {
        Ok(list) => {log::info!("Todos loaded successfully: {:?}, x:{}, y:{}", list,x,y);*preview_state.write() = PreviewState::Loaded(list.clone(),x,y,we_abuse_this_function_to_fetch_the_data_we_just_updated);},
        Err(err) =>{log::error!("Error loading todos: {}", err);*preview_state.write() = PreviewState::Error(err.to_string());},
       //None => {log::warn!("Todos are still loading");*preview_state.write() = PreviewState::Loading;}
    }

    //while !success{
        //match &*todos.read_unchecked() {
        //    Some(Ok(list)) => {log::info!("Todos loaded successfully: {:?}", list);*preview_state.write() = PreviewState::Loaded(list.clone());},
        //    Some(Err(err)) =>{log::error!("Error loading todos: {}", err);*preview_state.write() = PreviewState::Error(err.to_string());},
        //    None => {log::warn!("Todos are still loading");*preview_state.write() = PreviewState::Loading;}
        //}
    //}
    

}
//action_id: 1:toggle item done/undone, 2:move up, 3:move down, 4:create_new
async fn update_checklist(mut preview_state: Signal<PreviewState>,mut update_state: Signal<UpdateState>, x:i64, y:i64, todo_id:i64, action_id:i64, optional_title:String){
    *update_state.write() = UpdateState::Run;
    log::info!("updating");
    let url = format!("{}action/{}/{}/{}/{}/{}" ,BASE_API_URL,action_id, x,y,todo_id,optional_title);
    let response = reqwest::get(&url).await;
    match response{
        Ok(response) => {
            if response.status().is_success() {
                // Proceed with the original cell_click function
                //cell_click(x, y, preview_state);
                *update_state.write() = UpdateState::Done;
                cell_click(x, y, preview_state,true).await;
            } else {
                // Handle error
                log::info!("Failed to update JSON");
                *update_state.write() = UpdateState::Fail
            }
        }
        Err(e) => {
            // Handle error
            log::info!("Request error: {:?}", e);
            *update_state.write() = UpdateState::Fail
        }
    }
   
}

pub async fn get_module_preview(id: i64) -> Result<Module, reqwest::Error> {
    let url = format!("{}module_description/{}.json", BASE_API_URL, id);
    reqwest::get(&url).await?.json().await
}

pub async fn get_modules(semester: Semester) -> Result<Vec<Module>, reqwest::Error> {
    let url = format!("{}{}.json", BASE_API_URL, semester.to_string());
    let modules_ids = &reqwest::get(&url).await?.json::<Vec<i64>>().await?;

    let module_futures = modules_ids
        .iter()
        .map(|&module_id| get_module_preview(module_id));
    Ok(join_all(module_futures)
        .await
        .into_iter()
        .filter_map(|module| module.ok())
        .collect())
}

#[component]
fn ModuleBox(module: ReadOnlySignal<Module>) -> Element {
    //let mut preview_state = consume_context::<Signal<PreviewState>>();
    let Module { modulename, .. } = module();

    rsx! {
    div {
            a{"{modulename}"}
        }
    }
}
