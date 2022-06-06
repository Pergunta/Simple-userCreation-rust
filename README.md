# Rust webapp
With rocket and MongoDB

In order to setup your MongoDB service check https://www.mongodb.com/ , after creating a cluster you should see conections , get your connection string for rust, it should look something like this: 
```
mongodb+srv://<YOUR USERNAME HERE>:<YOUR PASSWORD HERE>@cluster0.e5akf.mongodb.net/myFirstDatabese?retryWrites=true&w=majority
```
Go ahead to the root of the rust project (Simple-userCreation-rust/manage-users) and create .env file, copy your connection string followed by  MONGOURI= 

## Run the project

At the root rust directory run :
```
cargo run
```
