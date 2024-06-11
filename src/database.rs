use anyhow::{anyhow, Error, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct MockDB {
    pub balances: Arc<Mutex<HashMap<String, f32>>>,
}

impl MockDB {
    pub async fn reset(&mut self) {
        self.balances.lock().await.clear();
    }
    pub async fn contains_key(&self, key: &str) -> bool {
        self.balances.lock().await.contains_key(key)
    }

    pub async fn balance(&self, origin: &str) -> Result<f32, Error> {
        match self.contains_key(origin).await {
            true => Ok(*self.balances.lock().await.get(origin).unwrap()),
            false => Err(anyhow!("does not contain key")),
        }
    }

    pub async fn deposit(&mut self, destination: &str, amount: f32) -> f32 {
        println!("{:#?}", self.balance(destination).await);
        self.balances
            .lock()
            .await
            .entry(destination.to_owned())
            .and_modify(|current| *current += amount)
            .or_insert(amount);

        println!("{:#?}", self.balance(destination).await);
        self.balance(destination).await.unwrap()
    }

    pub async fn withdraw(&mut self, origin: &str, amount: f32) -> Result<f32, Error> {
        println!("{:#?}", self.balance(origin).await);
        match self.balance(origin).await {
            Ok(x) if x < amount => Err(anyhow!("amount is greater than current balance")),
            Ok(x) => {
                self.balances
                    .lock()
                    .await
                    .entry(origin.to_owned())
                    .and_modify(|current| *current -= amount);
                println!("{:#?}", self.balance(origin).await);
                Ok(self.balance(origin).await.unwrap())
            }
            Err(err) => Err(err),
        }
    }

    pub async fn transfer(
        &mut self,
        origin: &str,
        destination: &str,
        amount: f32,
    ) -> Result<bool, Error> {
        match self.balance(origin).await {
            Ok(x) if x < amount => Err(anyhow!("amount is greater than current balance")),
            Ok(_) => {
                let _ = self.withdraw(origin, amount).await;
                let _ = self.deposit(destination, amount).await;
                Ok(true)
            }
            Err(err) => Err(err),
        }
    }
}

// self.balance(destination).unwrap()
