use crate::core::prelude::*;
use crate::filesystem::responses::CreateFilesystemResponse;
use crate::filesystem::{
    FilesystemRequired, FilesystemSupport, PropertiesOption, PropertiesSupport,
};
use azure_core::errors::AzureError;
use azure_core::{ClientRequestIdOption, ClientRequestIdSupport, TimeoutOption, TimeoutSupport};
use azure_core::{No, ToAssign, Yes};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateFilesystemBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_filesystem: PhantomData<FilesystemSet>,
    filesystem: Option<&'a str>,
    timeout: Option<u64>,
    properties: Option<&'a str>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> CreateFilesystemBuilder<'a, C, No>
where
    C: Client,
{
    pub(crate) fn new(client: &'a C) -> CreateFilesystemBuilder<'a, C, No> {
        CreateFilesystemBuilder {
            client,
            p_filesystem: PhantomData {},
            filesystem: None,
            timeout: None,
            properties: None,
            client_request_id: None,
        }
    }
}

impl<'a, C, FilesystemSet> ClientRequired<'a, C> for CreateFilesystemBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

impl<'a, C> FilesystemRequired<'a> for CreateFilesystemBuilder<'a, C, Yes>
where
    C: Client,
{
    #[inline]
    fn filesystem(&self) -> &'a str {
        self.filesystem.unwrap()
    }
}

impl<'a, C, FilesystemSet> TimeoutOption for CreateFilesystemBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, FilesystemSet> PropertiesOption<'a> for CreateFilesystemBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    #[inline]
    fn properties(&self) -> Option<&'a str> {
        self.properties
    }
}

impl<'a, C, FilesystemSet> ClientRequestIdOption<'a>
    for CreateFilesystemBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C> FilesystemSupport<'a> for CreateFilesystemBuilder<'a, C, No>
where
    C: Client,
{
    type O = CreateFilesystemBuilder<'a, C, Yes>;

    #[inline]
    fn with_filesystem(self, filesystem: &'a str) -> Self::O {
        CreateFilesystemBuilder {
            client: self.client,
            p_filesystem: PhantomData {},
            filesystem: Some(filesystem),
            timeout: self.timeout,
            properties: self.properties,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, FilesystemSet> TimeoutSupport for CreateFilesystemBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    type O = CreateFilesystemBuilder<'a, C, FilesystemSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        CreateFilesystemBuilder {
            client: self.client,
            p_filesystem: PhantomData {},
            filesystem: self.filesystem,
            timeout: Some(timeout),
            properties: self.properties,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, FilesystemSet> PropertiesSupport<'a> for CreateFilesystemBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    type O = CreateFilesystemBuilder<'a, C, FilesystemSet>;

    #[inline]
    fn with_properties(self, properties: &'a str) -> Self::O {
        CreateFilesystemBuilder {
            client: self.client,
            p_filesystem: PhantomData {},
            filesystem: self.filesystem,
            timeout: self.timeout,
            properties: Some(properties),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, FilesystemSet> ClientRequestIdSupport<'a>
    for CreateFilesystemBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    type O = CreateFilesystemBuilder<'a, C, FilesystemSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        CreateFilesystemBuilder {
            client: self.client,
            p_filesystem: PhantomData {},
            filesystem: self.filesystem,
            timeout: self.timeout,
            properties: self.properties,
            client_request_id: Some(client_request_id),
        }
    }
}

impl<'a, C, FilesystemSet> CreateFilesystemBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
}

impl<'a, C> CreateFilesystemBuilder<'a, C, Yes>
where
    C: Client,
{
    pub async fn finalize(self) -> Result<CreateFilesystemResponse, AzureError> {
        let mut uri = format!(
            "{}/{}?resource=filesystem",
            self.client().filesystem_uri(),
            self.filesystem()
        );

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let perform_request_response = self.client().perform_request(
            &uri,
            &Method::PUT,
            &|mut request| {
                request = ClientRequestIdOption::add_header(&self, request);
                request = PropertiesOption::add_header(&self, request);
                request
            },
            Some(&[]),
        )?;

        let (headers, _body) = perform_request_response
            .check_status_extract_headers_and_body(StatusCode::CREATED)
            .await?;
        CreateFilesystemResponse::from_headers(&headers)
    }
}
