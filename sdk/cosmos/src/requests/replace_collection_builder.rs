use crate::prelude::*;
use crate::resources::collection::{IndexingPolicy, PartitionKey};
use crate::responses::CreateCollectionResponse;
use azure_core::{No, ToAssign, Yes};
use http::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    collection_client: &'a CollectionClient,
    p_partition_key: PhantomData<PartitionKeysSet>,
    p_indexing_policy: PhantomData<IndexingPolicySet>,
    partition_key: Option<&'a PartitionKey>,
    indexing_policy: Option<&'a IndexingPolicy>,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> ReplaceCollectionBuilder<'a, 'b, No, No> {
    pub(crate) fn new(collection_client: &'a CollectionClient) -> Self {
        Self {
            collection_client,
            p_partition_key: PhantomData {},
            partition_key: None,
            p_indexing_policy: PhantomData {},
            indexing_policy: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, PartitionKeysSet, IndexingPolicySet>
    ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    pub fn collection_client(&self) -> &'a CollectionClient {
        self.collection_client
    }

    fn user_agent(&self) -> Option<azure_core::UserAgent<'b>> {
        self.user_agent
    }

    fn activity_id(&self) -> Option<azure_core::ActivityId<'b>> {
        self.activity_id
    }

    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }

    pub fn with_user_agent(self, user_agent: &'b str) -> Self {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }

    pub fn with_activity_id(self, activity_id: &'b str) -> Self {
        Self {
            activity_id: Some(azure_core::ActivityId::new(activity_id)),
            ..self
        }
    }

    pub fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> ReplaceCollectionBuilder<'a, 'b, Yes, Yes> {
    pub async fn execute(&self) -> Result<CreateCollectionResponse, CosmosError> {
        trace!("ReplaceCollectionBuilder::execute called");

        let req = self
            .collection_client
            .prepare_request_with_collection_name(http::Method::PUT);

        let req = crate::headers::add_header(self.user_agent(), req);
        let req = crate::headers::add_header(self.activity_id(), req);
        let req = crate::headers::add_header(self.consistency_level(), req);

        let req = req.header(http::header::CONTENT_TYPE, "application/json");

        #[derive(Debug, Clone, Serialize)]
        struct Request<'k> {
            id: &'k str,
            #[serde(rename = "indexingPolicy")]
            indexing_policy: &'k IndexingPolicy,
            #[serde(rename = "partitionKey")]
            partition_key: &'k PartitionKey,
        };

        let request = Request {
            id: self.collection_client().collection_name(),
            indexing_policy: self.indexing_policy(),
            partition_key: self.partition_key(),
        };

        let body = serde_json::to_string(&request)?;
        debug!("body == {}", body);

        let req = req.body(body.as_bytes())?;
        debug!("\nreq == {:?}", req);

        // the docs are wrong here
        // [https://docs.microsoft.com/en-us/rest/api/cosmos-db/replace-a-collection](https://docs.microsoft.com/en-us/rest/api/cosmos-db/replace-a-collection).
        // They say you should receive 201 instead azure returns 200 upon success. I've filed a PR
        // to correct it.
        Ok(self
            .collection_client
            .http_client()
            .execute_request_check_status(req, StatusCode::OK)
            .await?
            .try_into()?)
    }
}

impl<'a, 'b, IndexingPolicySet> ReplaceCollectionBuilder<'a, 'b, Yes, IndexingPolicySet>
where
    IndexingPolicySet: ToAssign,
{
    fn partition_key(&self) -> &'a PartitionKey {
        self.partition_key.unwrap()
    }
}

impl<'a, 'b, PartitionKeysSet> ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, Yes>
where
    PartitionKeysSet: ToAssign,
{
    fn indexing_policy(&self) -> &'a IndexingPolicy {
        self.indexing_policy.unwrap()
    }
}

impl<'a, 'b, IndexingPolicySet> ReplaceCollectionBuilder<'a, 'b, No, IndexingPolicySet>
where
    IndexingPolicySet: ToAssign,
{
    pub fn with_partition_key(
        self,
        partition_key: &'a PartitionKey,
    ) -> ReplaceCollectionBuilder<'a, 'b, Yes, IndexingPolicySet> {
        ReplaceCollectionBuilder {
            collection_client: self.collection_client,
            p_partition_key: PhantomData {},
            p_indexing_policy: PhantomData {},
            partition_key: Some(partition_key),
            indexing_policy: self.indexing_policy,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, PartitionKeysSet> ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, No>
where
    PartitionKeysSet: ToAssign,
{
    pub fn with_indexing_policy(
        self,
        indexing_policy: &'a IndexingPolicy,
    ) -> ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, Yes> {
        ReplaceCollectionBuilder {
            collection_client: self.collection_client,
            p_partition_key: PhantomData {},
            p_indexing_policy: PhantomData {},
            partition_key: self.partition_key,
            indexing_policy: Some(indexing_policy),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}
