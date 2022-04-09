use std::sync::Arc;
use neo4rs::{Graph, Node, query};

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

#[cfg(test)]
mod graph {
    use std::sync::Arc;
    use neo4rs::{Graph, query};
    use crate::graph_db::{example_query, example_transaction};

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