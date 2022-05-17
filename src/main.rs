mod type_request;
mod utils;

use std::env;
use threadpool::ThreadPool;
use reqwest::Client;
use reqwest::Error;
use tokio::runtime::Runtime;
use crate::type_request::{CreateItem, ItemPair, UpdateItem};
use crate::utils::*;

#[tokio::main]
async fn main() {
    // Start staffs to serve customers
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Specify number of clients: cargo run [num]");
    }
    let nb_client = (&args[1]).parse().unwrap();

    let pool = ThreadPool::with_name("clients".into(), nb_client);
    for i in 1..nb_client + 1 {
        pool.execute(move || {
            let runtime = Runtime::new().unwrap();
            loop {
                let type_query: i32 = random_numb(1,5);
                match type_query {
                    1 => match runtime.block_on(async {
                        post_item(i as i32).await
                    })
                    {
                        Ok(_) => {}
                        Err(e) => println!("Error : {}", e),
                    }
                    2 => match runtime.block_on(async {
                        get_item(random_numb(1,50)).await
                    })
                    {
                        Ok(_) => {}
                        Err(e) => println!("Error : {}", e),
                    }
                    3 => match runtime.block_on(async {
                        delete_item(random_numb(1,50)).await
                    })
                    {
                        Ok(_) => {}
                        Err(e) => println!("Error : {}", e),
                    }
                    4 => match runtime.block_on(async {
                        update_item(random_numb(1,50)).await
                    })
                    {
                        Ok(_) => {}
                        Err(e) => println!("Error : {}", e),
                    }
                    5 => match runtime.block_on(async {
                        get_items_by_table_id(i as i32).await
                    })
                    {
                        Ok(_) => {}
                        Err(e) => println!("Error : {}", e),
                    }
                    _ => println!("[ERROR] Number out limit !"),
                }
            }
        });
    }
    pool.join();
}

async fn post_item(tid: i32) -> Result<(), Error> {
    let id = tid.clone();
    let item: CreateItem = CreateItem {
        tid,
        items: item_randomizer(),
    };

    let url: String = "http://127.0.0.1:8080/items".to_string();
    let executor = Client::new();
    let resp = executor
        .post(url)
        .json(&item)
        .send()
        .await?;
    let msg = &resp.text().await?;
    println!("TABLE {} : CREATE ITEM :  {:?}", id, msg);
    Ok(())
}

async fn get_item(id: i32) -> Result<(), Error> {
    let mut url: String = "http://127.0.0.1:8080/items".to_string();
    url.push_str("/");
    url.push_str(&id.to_string());

    let executor = Client::new();
    let resp = executor
        .get(url)
        .send()
        .await?;
    let msg = resp.text().await?.to_string();
    println!("ITEM {} : {:?}", id, msg);
    Ok(())
}

async fn delete_item(id: i32) -> Result<(), Error> {
    let mut url: String = "http://127.0.0.1:8080/items".to_string();
    url.push_str("/");
    url.push_str(&id.to_string());

    let executor = Client::new();
    let resp = executor
        .delete(url)
        .send()
        .await?;
    let msg = resp.text().await?;
    println!("ITEM {} : ITEM DELETED {:?}", id, msg);
    Ok(())
}

async fn update_item(id: i32) -> Result<(), Error> {
    let item: UpdateItem = UpdateItem {
        name: random_string(),
        cook_time: random_numb(5,15),
    };

    let mut url: String = "http://127.0.0.1:8080/items".to_string();
    url.push_str("/");
    url.push_str(&id.to_string());

    let executor = Client::new();
    let resp = executor
        .patch(url)
        .json(&item)
        .send()
        .await?;
    let msg = resp.text().await?.to_string();
    println!("ITEM {} : ITEM UPDATED : {:?}", id, msg);
    Ok(())
}

async fn get_items_by_table_id(tid: i32) -> Result<(), Error> {
    let mut url: String = "http://127.0.0.1:8080/tables".to_string();
    url.push_str("/");
    url.push_str(&tid.to_string());

    let executor = Client::new();
    let resp = executor
        .get(url)
        .send()
        .await?;
    let msg = resp.text().await?.to_string();
    println!("TABLE {} : ITEM(S) : {:?}", tid, msg);
    Ok(())
}