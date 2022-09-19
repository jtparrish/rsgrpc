use tonic::{transport::Server, Request, Response, Status};
use service::file_manager_client::FileManagerClient;
use service::{FileName, PutFileMsg};
use std::env;
// use std::io::Result as ioResult;

// type ResponseResult<T> = Result<Response<T>, Status>;
type CommandResult = Result<String, Status>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::channel::Channel::from_static("https://127.0.0.1:50051")
        .connect()
        .await?;
    
    let mut client = FileManagerClient::new(channel);

    let mut args = env::args();
    args.next();

    let res: String = match args.next().as_ref().map(|s| s.as_str()) {
        Some("create")  => create_file( &mut client, args.next() ).await?,
        Some("read")    => read_file( &mut client, args.next() ).await?,
        Some("write")   => write_file( &mut client, args.next(), args.next() ).await?,
        _               => usage_msg(),
    };
    println!("{}", res);

    Ok( () )
}

async fn create_file(client: &mut FileManagerClient<tonic::transport::channel::Channel>, fname: Option<String>) -> CommandResult {
    Ok(match fname {
        Some(s) => format!("{:?}", client.make_file(tonic::Request::new(
            FileName {
                name: s,
            }
        )).await?.into_inner()),
        None => usage_msg(),
    })
}

async fn read_file(client: &mut FileManagerClient<tonic::transport::channel::Channel>, fname: Option<String>) -> CommandResult {
    Ok(match fname {
        Some(f) => format!("{:?}", client.get_file_contents(tonic::Request::new(
            FileName {
                name: f,
            }
        )).await?.into_inner()),
        None => usage_msg(),
    })
}

async fn write_file(client: &mut FileManagerClient<tonic::transport::channel::Channel>, fname: Option<String>, content: Option<String>) -> CommandResult {
    Ok(match (fname, content) {
        (Some(f), Some(c)) => format!("{:?}", client.put_file_contents(tonic::Request::new(
            PutFileMsg {
                name: f,
                contents: c,
            }
        )).await?.into_inner()),
        _ => usage_msg(),
    })
}

fn usage_msg() -> String {
    "USAGE BAD".to_owned()
}