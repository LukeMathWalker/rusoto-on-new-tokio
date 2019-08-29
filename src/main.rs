use rusoto_core::Region;
use rusoto_firehose::{KinesisFirehoseClient, PutRecordInput, KinesisFirehose};
use rusoto_mock::{MockCredentialsProvider, MockRequestDispatcher};
use futures::compat::Future01CompatExt;

fn main() {
    let client = KinesisFirehoseClient::new(Region::EuWest1);
    let stream_name = "a-test-stream";

    let firehose_record = PutRecordInput {
        delivery_stream_name: stream_name.to_string(),
        record: rusoto_firehose::Record {
            data: "random stuff".as_bytes().into(),
        },
    };
    let rusoto_future = client.put_record(firehose_record).compat();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async { rusoto_future.await.unwrap(); });
}
