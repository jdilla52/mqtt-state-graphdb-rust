use crate::config::GraphdbSettings;
use log::debug;
use neo4rs::{query, Graph, Node};
use std::fmt::Display;
use std::io::Bytes;
use std::sync::Arc;

async fn example_query(graph: Arc<Graph>) {
    for _ in 1..=42 {
        let graph = graph.clone();
        tokio::spawn(async move {
            let mut result = graph
                .execute(query("MATCH (p:Person {name: $name}) RETURN p").param("name", "Mark"))
                .await
                .unwrap();
            while let Ok(Some(row)) = result.next().await {
                let node: Node = row.get("p").unwrap();
                let name: String = node.get("name").unwrap();
                println!("{}", name);
            }
        });
    }
}

async fn example_transaction(graph: Arc<Graph>) {
    //Transactions
    let mut txn = graph.start_txn().await.unwrap();
    txn.run_queries(vec![
        query("CREATE (p:Person {name: 'mark'})"),
        query("CREATE (p:Person {name: 'jake'})"),
        query("CREATE (p:Person {name: 'luke'})"),
    ])
    .await
    .unwrap();
    txn.commit().await.unwrap();
}

#[derive(Clone)]
pub struct Graphdb {
    settings: GraphdbSettings,
    graph: Arc<Graph>,
}

impl Graphdb {
    pub async fn new(settings: GraphdbSettings) -> Graphdb {
        let graph = Arc::new(
            Graph::new(&settings.address, &settings.user, &settings.pass)
                .await
                .unwrap(),
        );
        Graphdb { settings, graph }
    }

    pub async fn create_path(&self, topic: String, data: Vec<u8>) -> neo4rs::Result<()> {
        let mut txn = self.graph.start_txn().await.unwrap();

        let merges: Vec<&str> = topic.split("/").collect();
        let mut pattern = "MERGE (root :ROOT {name: 'root'})".to_string(); // root

        // this will result in duplicate dodes where if a user places a value halfway through the path.
        // I think this is fine and should make queries substantially faster as we can just grab all values.
        let m0 = merges[0];
        // let vn0 = var_names[0];
        if merges.len() == 1 {
            let name = format!("{}_{}", "v", 0);
            pattern = pattern + &format!(" MERGE ({n} :DATA {{name: '{v}'}})", n = name, v = m0);
            pattern = pattern + &format!(" MERGE (root) <-[:SUB]- ({v})", v = name);
        // connect two above
        } else {
            // first node connecting to root
            pattern = pattern + &format!(" MERGE (v{n} :PATH {{name: '{d}'}})", n = 0, d = m0);
            pattern = pattern + &format!(" MERGE (root) <-[:SUB]- (v{v})", v = 0);

            // iterate nodes conecting to previous
            for (i, v) in (&merges[1..merges.len() - 1]).iter().enumerate() {
                // current node
                pattern =
                    pattern + &format!(" MERGE (v{n} :PATH {{name: '{v}'}})", n = i + 1, v = v);
                // connect current to previous node i
                pattern =
                    pattern + &format!(" MERGE (v{v0}) <-[:SUB]- (v{v1})", v0 = i, v1 = i + 1);
            }

            // last node as it has data
            let ml = *merges.last().unwrap();
            let l = merges.len() - 1;
            pattern = pattern + &format!(" MERGE (v{vn} :DATA {{name: '{name}', data: '{data}'}})", vn = l, name=ml, data = "lkjlkj"); // first node
            pattern = pattern + &format!(" MERGE (v{v0}) <-[:SUB]- (v{v1})", v0 = l - 1, v1 = l);
            // connect
        }

        debug!("pattern: {}", pattern);

        txn.run(query(pattern.as_str())).await.unwrap();
        txn.commit().await
    }
}

#[cfg(test)]
mod graph {
    use crate::config::GraphdbSettings;
    use crate::graph_db::{example_query, example_transaction, Graphdb};
    use neo4rs::{query, Graph};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_create_single_path() {
        let client = Graphdb::new(GraphdbSettings::default()).await;
        // test single path
        client
            .create_path("test".to_string(), "hello".as_bytes().to_vec())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_create_deep_path() {
        let client = Graphdb::new(GraphdbSettings::default()).await;
        // test single path
        let r = client
            .create_path(
                "test/test2/hello".to_string(),
                "{name: 'value', test_val:'hello'}".as_bytes().to_vec(),
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_create_deep_path_duplicate() {
        let client = Graphdb::new(GraphdbSettings::default()).await;
        // test single path
        let r = client
            .create_path(
                "test/tes5/test/test5".to_string(),
                "{name: 'value', test_val:'hello'}".as_bytes().to_vec(),
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn transaction() {
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
