<h1 align="center">🥢 Welcome to openmensa-rs 🍴</h1>
<p align="center">
  <a href="https://travis-ci.org/jwuensche/openmensa-rs">
    <img src="https://api.travis-ci.org/jwuensche/openmensa-rs.svg?branch=master">
  </a>
  <a href="https://github.com/jwuensche/openmensa-rs/blob/master/LICENSE">
    <img alt="GitHub" src="https://img.shields.io/github/license/jwuensche/openmensa-rs.svg">
  </a>
  <a href="http://makeapullrequest.com">
    <img alt="PRs Welcome" src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg" target="_blank" />
  </a>
  <br>
  <i>rust library to interact with the <a href="https://openmensa.org/">openmensa</a> api </i>
</p>

---

> :exclamation: This project requires rust >= 1.40

## How to use 🔧

This library provides a way to build request to the openmensa api and deserialize into rust structures.
Provided are requests for `Canteen`, `Meal` and `Day`.

### Requesting a list of all canteens

```rust 
use openmensa_rs::CanteenRequest;

#[tokio::main]
async fn main() {
    println!("List of all canteens: ");
    let list = CanteenRequest::new().build().await;
    println!("{:?}", list);
}
```

### Requesting the meals for a single canteen

```rust
use openmensa_rs::{req_canteens, MealRequest};

#[tokio::main]
async fn main() {
    let list = req_canteens().await.unwrap();
    // Print out the meals offered in the first canteen
    println!("Meals in the {} canteen on the {}", list[0].name, chrono::Utc::today());
    let meals = MealRequest::new(list[0].id, chrono::Utc::today())
        .build()
        .await
        .unwrap();
    println!("{:?}", meals);
}
```

### Shorthands for generic Requests

If you want to get all information and specify no further constraints for your requests, you can use the provided shorthands `req_canteens`, `req_days` and `req_meals`.

```rust
use openmensa_rs::req_canteens;

#[tokio::main]
async fn main() {
    let list = req_canteens().await.unwrap();
    println!("First canteens has id {} and is called {}", list[0].id, list[0].name);
}
```

## Add this crate 📦

All you have to do is add in your `Cargo.toml` under `dependencies`
```toml
[dependencies]
openmensa-rs = "^0"
```


## Troubleshooting

If you have any troubles using this library or you find any problems feel free to open an issue. Explaining your problem and if possible provide a short code snippet.

## Authors

**Johannes Wünsche**

> [:octocat: `@jwuensche`](https://github.com/jwuensche)  
> [:elephant: `@fredowald@mastodon.social`](https://mastodon.social/web/accounts/843376)  
> [:bird: `@Fredowald`](https://twitter.com/fredowald)  

## Show your support

Give a :star: if this project helped you!
