use std::sync::Arc;
use log::debug;
use neo4rs::{Graph, Node, query};
use crate::config::GraphdbSettings;

async fn example_query(graph: Arc<Graph>){
    for _ in 1..=42 {
        let graph = graph.clone();
        tokio::spawn(async move {
            let mut result = graph.execute(
                query("MATCH (p:Person {name: $name}) RETURN p").param("name", "Mark")
            ).await.unwrap();
            while let Ok(Some(row)) = result.next().await {
                let node: Node = row.get("p").unwrap();
                let name: String = node.get("name").unwrap();
                println!("{}", name);
            }
        });
    }
}

async fn example_transaction(graph: Arc<Graph>){
    //Transactions
    let mut txn = graph.start_txn().await.unwrap();
    txn.run_queries(vec![
        query("CREATE (p:Person {name: 'mark'})"),
        query("CREATE (p:Person {name: 'jake'})"),
        query("CREATE (p:Person {name: 'luke'})"),
    ])
        .await
        .unwrap();
    txn.commit().await.unwrap(); //or txn.rollback().await.unwrap();
}

pub struct Graphdb {
    settings: GraphdbSettings,
    graph: Arc<Graph>
}

impl Graphdb {
    pub async fn new(settings: GraphdbSettings) -> Graphdb {
        let graph = Arc::new(Graph::new(&settings.address, &settings.user, &settings.pass).await.unwrap());
        Graphdb{
            settings,
            graph
        }
    }

    pub async fn create_path(&self, topic: String, data: String){
        let mut txn = self.graph.start_txn().await.unwrap();


        let merges: Vec<&str> = topic.split("/").collect();
        let mut pattern = "MERGE (root :ROOT {name: 'root'})".to_string(); // root

        // this will result in duplicate dodes where if a user places a value halfway through the path.
        // I think this is fine and should make queries substantially faster as we can just grab all values.

        let m0 = merges[0];
        if merges.len() == 1{
            pattern = pattern + &format!(" MERGE ({v} :Value {{name: '{v}'}})", v = m0); // first node
            pattern = pattern + &format!(" MERGE (root) <-[:SUB]- ({v})", v=m0); // connect two above
        }
        else {
            pattern =pattern + &format!(" MERGE ({v} :Path {{subpath: '{v}'}})", v=m0); // first node
            pattern =pattern + &format!(" MERGE (root) <-[:SUB]- ({v})", v=m0); // connect two above
        
            for (i, v) in (&merges[1..merges.len()-1]).iter().enumerate(){
                pattern = pattern + &format!(" MERGE ({v} :Path {{subpath: '{v}'}})", v=v);
                pattern = pattern + &format!(" MERGE ({v0}) <-[:SUB]- ({v1})", v0=v, v1=merges[i]); // connect current to previous
            }
            let ml = merges.last().unwrap();
            pattern = pattern + &format!(" MERGE ({v} :Value {data})", v = ml, data=data); // first node // TODO add actual value
            pattern = pattern + &format!(" MERGE ({v0}) <-[:SUB]- ({v1})", v0=merges[merges.len()-2], v1=ml); // connect
        }

        debug!("pattern: {}", pattern);

        txn.run(query( pattern.as_str()))
            .await
            .unwrap();
        txn.commit().await.unwrap(); //or txn.rollback().await.unwrap();
    }
}

#[cfg(test)]
mod graph {
    use std::sync::Arc;
    use neo4rs::{Graph, query};
    use crate::config::GraphdbSettings;
    use crate::graph_db::{example_query, example_transaction, Graphdb};

    #[tokio::test]
    async fn test_create_single_path() {
        let client = Graphdb::new(GraphdbSettings::default()).await;
        // test single path
        client.create_path("test".to_string(), "hello".to_string()).await;
    }

    #[tokio::test]
    async fn test_create_deep_path() {
        let client = Graphdb::new(GraphdbSettings::default()).await;
        // test single path
        client.create_path("test/test2/hello".to_string(), "{name: 'value', test_val:'hello'}".to_string()).await;
    }

    #[tokio::test]
    async fn transaction () {
        let uri = "127.0.0.1:7687";
        let user = "neo4j";
        let pass = "test";
        let graph = Arc::new(Graph::new(&uri, user, pass).await.unwrap());
        example_transaction(graph).await;
    }
    #[tokio::test]
    async fn test_query() {
        let uri = "127.0.0.1:7687";
        let user = "neo4j";
        let pass = "test";
        let graph = Arc::new(Graph::new(&uri, user, pass).await.unwrap());
        example_query(graph).await;
    }

}