use std::sync::Arc;
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

        txn.run(query( "MERGE (n1 :Person {name: '1'})\
                               MERGE (n2 :Person {name: '2'})\
                               MERGE (n1) <-[:SUB]- (n2)"))
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
    async fn test_create_path() {
        let client = Graphdb::new(GraphdbSettings::default()).await;
        client.create_path("test/tes2/test_value".to_string(), "hello".to_string()).await;
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