#![feature(async_fn_in_trait)]

use crate::ec2::CreateInstance;
trait Request<T> {
    fn get_mut(&mut self) -> &mut T;
    fn set_fields(&mut self, t: impl Into<T>) -> &mut Self;
    async fn send(self) -> Response;
}

struct Response;

struct AWSSdkCredentials {
    // credentials fields here
}

pub mod ec2 {
    use super::*;

    #[derive(Default)]
    pub struct CreateInstance;
    #[derive(Default)]
    pub struct DeleteInstance;

    pub struct EC2<T> {
        credentials: AWSSdkCredentials,
        parameters: T,
    }

    pub fn ec2_new<T: Default>() -> EC2<T> {
        EC2 {
            credentials: AWSSdkCredentials{},
            parameters: Default::default()
        }
    }

    impl<T> Request<T> for EC2<T> {
        fn get_mut(&mut self) -> &mut T {
            &mut self.parameters
        }
        fn set_fields(&mut self, t: impl Into<T>) -> &mut Self {
            self.parameters = t.into();
            self
        }
        async fn send(self) -> Response {
            let http_response = {
                // do some http request?
                Response
            };
            http_response
        }
    }
}

pub mod s3 {
    use super::*;

    struct PutObject;
    struct GetObject;
    struct S3<T> {
        credentials: AWSSdkCredentials,
        parameters: T,
    }
    impl<T> Request<T> for S3<T> {
        fn get_mut(&mut self) -> &mut T {
            &mut self.parameters
        }
        fn set_fields(&mut self, t: impl Into<T>) -> &mut Self {
            self.parameters = t.into();
            self
        }
        async fn send(self) -> Response {
            let http_response = {
                // do some http request?
                Response
            };
            http_response
        }
    }
}

fn main() {
    let mut ec2 = ec2::ec2_new::<CreateInstance>();
    ec2
        .get_mut()
        /*
            call some functions to set parameters
        */
        ; 
    // over ride parameters
    ec2.set_fields(CreateInstance);
    ec2.send();
    println!("Hello, world!");
}
