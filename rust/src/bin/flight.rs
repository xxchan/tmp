use std::pin::Pin;

use arrow_flight::flight_service_server::{FlightService, FlightServiceServer};
use arrow_flight::{
    Action, ActionType, Criteria, Empty, FlightData, FlightDescriptor, FlightInfo,
    HandshakeRequest, HandshakeResponse, PutResult, SchemaAsIpc, SchemaResult, Ticket,
};
use arrow_ipc::writer::IpcWriteOptions;
use arrow_schema::Field;
use futures::Stream;
use tonic::transport::Server;
use tonic::{Request, Response, Status, Streaming};

#[derive(Clone)]
pub struct FlightServiceImpl {}

type TonicStream<T> = Pin<Box<dyn Stream<Item = Result<T, tonic::Status>> + Send + 'static>>;

#[async_trait::async_trait]
impl FlightService for FlightServiceImpl {
    type HandshakeStream = TonicStream<HandshakeResponse>;
    type ListFlightsStream = TonicStream<FlightInfo>;
    type DoGetStream = TonicStream<FlightData>;
    type DoPutStream = TonicStream<PutResult>;
    type DoActionStream = TonicStream<arrow_flight::Result>;
    type ListActionsStream = TonicStream<ActionType>;
    type DoExchangeStream = TonicStream<FlightData>;

    async fn handshake(
        &self,
        request: Request<Streaming<HandshakeRequest>>,
    ) -> Result<Response<Self::HandshakeStream>, Status> {
        let request = request.into_inner().message().await?.unwrap();
        let response = HandshakeResponse {
            protocol_version: request.protocol_version,
            payload: request.payload,
        };
        let output = futures::stream::iter(std::iter::once(Ok(response)));
        Ok(Response::new(Box::pin(output) as Self::HandshakeStream))
    }

    async fn list_flights(
        &self,
        _request: Request<Criteria>,
    ) -> Result<Response<Self::ListFlightsStream>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    // create function foo(int) returns int as 'foo' using link 'http://localhost:6666';

    async fn get_flight_info(
        &self,
        _request: Request<FlightDescriptor>,
    ) -> Result<Response<FlightInfo>, Status> {
        loop{}
        let schema =
            arrow_schema::Schema::new(vec![Field::new("a", arrow_schema::DataType::Int32, false)]);
        let schema: SchemaResult = SchemaAsIpc::new(&schema, &IpcWriteOptions::default())
            .try_into()
            .unwrap();

        let info = FlightInfo {
            schema: schema.schema,
            total_records: 2, // WRONG VALUE
            ..Default::default()
        };
        Ok(Response::new(info))
    }

    async fn get_schema(
        &self,
        _request: Request<FlightDescriptor>,
    ) -> Result<Response<SchemaResult>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn do_get(
        &self,
        _request: Request<Ticket>,
    ) -> Result<Response<Self::DoGetStream>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn do_put(
        &self,
        _request: Request<Streaming<FlightData>>,
    ) -> Result<Response<Self::DoPutStream>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn do_action(
        &self,
        _request: Request<Action>,
    ) -> Result<Response<Self::DoActionStream>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn list_actions(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::ListActionsStream>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn do_exchange(
        &self,
        _request: Request<Streaming<FlightData>>,
    ) -> Result<Response<Self::DoExchangeStream>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:6666".parse()?;
    let service = FlightServiceImpl {};

    let svc = FlightServiceServer::new(service);
    println!("Flight service listening on {}", addr);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
