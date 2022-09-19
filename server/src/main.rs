mod fs_backend;

use tonic::{transport::Server, Request, Response, Status};
use service::file_manager_server::{FileManager, FileManagerServer};
use service::{FileName, PutFileMsg, FileContentResponse, Success};

use fs_backend::{create_file, read_file, write_file};

type ResponseResult<T> = Result<Response<T>, Status>;

struct ServerImpl;

// TODO: fix this stuff to actually do something
#[tonic::async_trait]
impl FileManager for ServerImpl {
    async fn make_file(&self, request: Request<FileName>) -> ResponseResult<Success> {
        Ok(Response::new(create_file(request.into_inner())))
    }
    async fn put_file_contents(&self, request: Request<PutFileMsg>) -> ResponseResult<Success> {
        Ok(Response::new(write_file(request.into_inner())))
    }
    async fn get_file_contents(&self, request: Request<FileName>) -> ResponseResult<FileContentResponse> {
        Ok(Response::new(read_file(request.into_inner())))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // unimplemented!();
    // let addr = "localhost:50051".parse().expect("ip bad");
    // let addr = "localhost:50051".to_socket_addrs().expect("socket addrs bad").next().expect("something is wrong with iterator");
    let addr = "0.0.0.0:50051".parse().expect("ip bad");
    // let addr = "[::1]:50051".parse().expect("ip bad");
    let svr = ServerImpl;
    println!("Server listening on address {}", addr);
    Server::builder()
        .add_service(FileManagerServer::new(svr))
        .serve(addr)
        .await?;

    Ok( () )
}
