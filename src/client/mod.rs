use rusoto_core::{Region};
use rusoto_s3::{GetObjectRequest, ListObjectsV2Request,
                PutObjectRequest, Object, S3, S3Client};

const MANIFEST_PREFIX: &'static str = "manifest/";
const RESOURCE_PREFIX: &'static str = "resources/";

// TODO: Caching

pub struct BackendClient {
    s3_client: S3Client,
    bucket_name: String
}

impl BackendClient {
    pub fn new(bucket_name: String, region: Region) -> Self {
        // TODO: Use async support for the S3 client?
        BackendClient {
            s3_client: S3Client::simple(region),
            bucket_name: bucket_name,
        }
    }

    pub fn list(&self) -> Result<Vec<Manifest>, ()> {
        let mut request: ListObjectsV2Request = Default::default();
        request.bucket = self.bucket_name.clone();
        request.prefix = Some(MANIFEST_PREFIX.to_string());

        let mut output: Vec<Manifest> = Vec::new();

        loop {
            match self.s3_client.list_objects_v2(&request).sync() {
                Ok(resp) => {
                    output.append(&mut resp.contents.unwrap_or(Vec::new())
                       .into_iter().filter_map(|x| {
                           self.obj_to_manifest(x, MANIFEST_PREFIX)
                       }).collect());

                    match resp.next_continuation_token {
                        Some(token) => { request.continuation_token = Some(token); },
                        None => { break; }
                    }
                },
                Err(err) => {
                    println!("Received S3 error calling list: {}", err);
                    return Err(())
                }
            }
        }

        Ok(output)
    }

    pub fn get(&self, manifest: Manifest) -> Result<Data, ()> {
        let mut request: GetObjectRequest = Default::default();
        request.bucket = self.bucket_name.clone();
        request.key = RESOURCE_PREFIX.to_string() + &manifest.name;

        match self.s3_client.get_object(&request).sync() {
            Ok(_) => {
                Ok(Data{
                    manifest: manifest,
                })
            },
            Err(_) => {
                Err(())
            }
        }
    }

    pub fn put(&self, data: Data) -> Result<(), ()> {
        // First, we need to put the data, and then we put the manifest
        let mut request: PutObjectRequest = Default::default();
        request.bucket = self.bucket_name.clone();

        // TODO: Implement a way to stream the bytes into S3.
        request.key = RESOURCE_PREFIX.to_string() + &data.manifest.name;
        request.body = None;
        try!(self.convert_result(self.s3_client.put_object(&request).sync()));

        request.key = MANIFEST_PREFIX.to_string() + &data.manifest.name;
        request.body = None;
        try!(self.convert_result(self.s3_client.put_object(&request).sync()));

        Ok(())
    }

    fn convert_result<T, E>(&self, result: Result<T, E>) -> Result<(), ()> {
        match result {
            Ok(_) => { Ok(()) },
            Err(_) => {
                // TODO: Print out information about the result.
                Err(())
            }
        }
    }

    fn obj_to_manifest(&self, obj: Object, prefix: &str) -> Option<Manifest> {
        obj.key.map(|key| {
            Manifest {
               name: key.trim_left_matches(prefix).to_string(),
            }
        })
    }
}

#[derive(Debug)]
pub struct Manifest {
    name: String,
}

#[derive(Debug)]
pub struct Data {
    manifest: Manifest,
}
