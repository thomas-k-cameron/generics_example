#![feature(async_fn_in_trait)]

use name_conflict::NameConflict;

use crate::ec2::CreateInstance;

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
}

fn name_conflict() {
    let mut nc = NameConflict(());
    let mut_ref = Request::set_fields(&mut nc, ());
}

trait Request<T> {
    fn get_mut(&mut self) -> &mut T;
    fn set_fields(&mut self, t: impl Into<T>) -> &mut Self;
    async fn send(self) -> Response;
}

struct Response;

struct AWSSdkCredentials {
    // credentials fields here
}

pub mod name_conflict {
    use super::*;
    pub struct NameConflict<T>(pub T);
    impl<T> NameConflict<T> {
        pub fn set_fields(&mut self, _: ()) {}
    }

    impl<T> Request<T> for NameConflict<T> {
        fn get_mut(&mut self) -> &mut T {
            &mut self.0
        }
        fn set_fields(&mut self, _: impl Into<T>) -> &mut Self {
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

    pub struct PutObject;
    pub struct GetObject;
    pub struct S3<T> {
        credentials: AWSSdkCredentials,
        parameters: T,
    }

    pub fn s3_new<T: Default>() -> S3<T> {
        S3 {
            credentials: AWSSdkCredentials{},
            parameters: Default::default()
        }
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

