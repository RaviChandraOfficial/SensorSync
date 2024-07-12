
use aws_iot_device_sdk_rust::{async_event_loop_listener, AWSIoTAsyncClient, AWSIoTSettings};
use rumqttc::{self, Packet, QoS};
use serde::{Deserialize, Serialize};
use std::error::Error;
use calamine::{open_workbook, Data, Reader, Xlsx, XlsxError};
use tokio::time::{sleep, Duration};

#[derive(Serialize, Deserialize)]
pub struct Message {
    text: String,
}

async fn publish_excel_data(iot_core_client: &AWSIoTAsyncClient, path: &str) -> Result<(), Box<dyn Error>> {
    // Open the workbook
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    // Specify the sheet name
    let sheet_name = "Sheet1"; // Replace with your sheet name

    // Read data from the Excel sheet
    let mut excel_data = String::new();

    // Get the range of the specified sheet and extract data
    match workbook.worksheet_range(sheet_name) {
        Ok(range) => {
            println!("Data in '{}':", sheet_name);
            for row in range.rows() {
                for cell in row {
                    print!("{:?}\t", cell);
                    excel_data.push_str(&format!("{:?}\t", cell));
                }
                println!();
            }

            // Provide some statistics
            let total_cells = range.get_size().0 * range.get_size().1;
            let non_empty_cells: usize = range.used_cells().count();
            println!(
                "Found {} cells in '{}', including {} non-empty cells",
                total_cells, sheet_name, non_empty_cells
            );
        }
        Err(e) => {
            println!("Error reading the workbook: {:?}", e);
            return Err(Box::new(e));
        }
    }

    // Publish the Excel data to AWS IoT
    let message = Message { text: excel_data };
    let json_message = serde_json::to_string(&message)?;
    match iot_core_client.publish("test1234".to_string(), QoS::AtMostOnce, &*json_message).await {
        Ok(_) => println!("Message published successfully"),
        Err(e) => println!("Error publishing message: {:?}", e),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // List of Excel file paths
    let file_paths = vec![
        "********************************************************.xlsx",
        "********************************************************.xlsx",
        "********************************************************.xlsx",
    ];

    let aws_settings = AWSIoTSettings::new(
        "******".to_owned(),
        "********************************************************".to_owned(),
        "********************************************************".to_owned(),
        "********************************************************".to_owned(),
        "********************************************************".to_owned(),
        None,
    );

    let (iot_core_client, eventloop_stuff) = AWSIoTAsyncClient::new(aws_settings).await.unwrap();

    iot_core_client.subscribe("*******".to_string(), QoS::AtMostOnce).await.unwrap();
    iot_core_client.publish("******".to_string(), QoS::AtMostOnce, "hey").await.unwrap();

    let mut receiver1 = iot_core_client.get_receiver().await;
    let mut receiver2 = iot_core_client.get_receiver().await;

    let recv1_thread = tokio::spawn(async move {
        loop {
            match receiver1.recv().await {
                Ok(event) => {
                    match event {
                        Packet::Publish(p) => {
                            println!("Received message {:?} on topic: {}", p.payload, p.topic)
                        }
                        _ => println!("Got event on receiver1: {:?}", event),
                    }
                }
                Err(e) => {
                    println!("Error receiving message: {:?}", e);
                }
            }
        }
    });

    let recv2_thread = tokio::spawn(async move {
        loop {
            match receiver2.recv().await {
                Ok(event) => println!("Got event on receiver2 : {:?}", event),
                Err(e) => {
                    println!("Error receiving message: {:?}", e);
                }
            }
        }
    });

    let publish = tokio::spawn(async move {
        loop {
            for path in &file_paths {
                match publish_excel_data(&iot_core_client, path).await {
                    Ok(_) => println!("Data from {} published successfully", path),
                    Err(e) => println!("Error publishing data from {}: {:?}", path, e),
                }
            }
            sleep(Duration::from_secs(5)).await; // Add a delay to avoid rapid continuous publishing
        }
    });
    
    let listen_thread = tokio::spawn(async move {
        async_event_loop_listener(eventloop_stuff).await.unwrap();
    });

    tokio::join!(
        recv1_thread, 
        recv2_thread, 
        listen_thread,
        publish
    );

    Ok(())
}




