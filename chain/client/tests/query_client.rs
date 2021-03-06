use actix::System;
use futures::{future, FutureExt};

use near_client::test_utils::setup_no_network;
use near_client::Query;
use near_logger_utils::init_test_logger;
use near_primitives::types::BlockIdOrFinality;
use near_primitives::views::{QueryRequest, QueryResponseKind};

/// Query account from view client
#[test]
fn query_client() {
    init_test_logger();
    System::run(|| {
        let (_, view_client) = setup_no_network(vec!["test"], "other", true, true);
        actix::spawn(
            view_client
                .send(Query::new(
                    BlockIdOrFinality::latest(),
                    QueryRequest::ViewAccount { account_id: "test".to_owned() },
                ))
                .then(|res| {
                    match res.unwrap().unwrap().unwrap().kind {
                        QueryResponseKind::ViewAccount(_) => (),
                        _ => panic!("Invalid response"),
                    }
                    System::current().stop();
                    future::ready(())
                }),
        );
    })
    .unwrap();
}
