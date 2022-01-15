use anyhow::Result;
use dotenv::dotenv;

fn main() -> Result<()> {
    dotenv().ok();
    println!("Hello, world!");
    async_global_executor::block_on(async{
        Ok(())
    })
}
