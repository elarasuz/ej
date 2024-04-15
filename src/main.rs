use std::fs::File;

use chrono::prelude::*;
use polars::{df, frame::DataFrame, io::{csv::CsvWriter, SerWriter}};

#[tokio::main]
async fn main() {
    let mut df: DataFrame = df!(
        "integer" => &[1, 2, 3],
        "date" => &[
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                NaiveDate::from_ymd_opt(2025, 1, 2).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                NaiveDate::from_ymd_opt(2025, 1, 3).unwrap().and_hms_opt(0, 0, 0).unwrap(),
        ],
        "float" => &[4.0, 5.0, 6.0],
        "string" => &["a", "b", "c"],
    )
    .unwrap();
    println!("{}", df);
    let mut file = File::create("examples/output/test.csv").expect("could not create file");
    CsvWriter::new(&mut file).finish(&mut df).unwrap();

    // ParquetWriter::new(&mut file).finish(&mut df).unwrap();
    // CsvWriter::new(&mut file)
    //     .include_header(true)
    //     .with_separator(b',')
    //     .finish(&mut df)?;
    // let df_csv = CsvReader::from_path("examples/output/test.csv")?
    //     .infer_schema(None)
    //     .has_header(true)
    //     .finish()?;
    // println!("{}", df_csv);
}

// use bleasy::{Error, ScanConfig, Scanner};
// use futures::StreamExt;
// use std::sync::atomic::{AtomicU32, Ordering};
// use std::sync::Arc;
// use tokio::time::{sleep, Duration};

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     pretty_env_logger::init();

//     // Create a new BLE device scanner
//     let mut scanner = Scanner::new();

//     // Start the scanner with default configuration
//     scanner.start(ScanConfig::default()).await?;

//     // Create a stream that is provided with discovered devices
//     let mut device_stream = scanner.device_stream();

//     // Create a thread-safe counter
//     let count = Arc::new(AtomicU32::new(0));

//     // List devices in a separate thread as they are discovered
//     let join_handle = {
//         let count = count.clone();
//         tokio::spawn(async move {
//             while let Some(device) = device_stream.next().await {
//                 println!("Found device with name {:?}", device.local_name().await);
//                 count.fetch_add(1, Ordering::SeqCst);
//             }
//         })
//     };

//     // Wait until at least two devices are found
//     while count.load(Ordering::SeqCst) < 2 {
//         sleep(Duration::from_millis(100)).await;
//     }

//     // Stop the scanner after 2 devices are found
//     scanner.stop().await?;

//     join_handle.await.unwrap();

//     Ok(())
// }

// See the "macOS permissions note" in README.md before running this on macOS
// Big Sur or later.

// use btleplug::api::{Central, CharPropFlags, Manager as _, Peripheral, ScanFilter};
// use btleplug::platform::Manager;
// use futures::stream::StreamExt;
// use std::error::Error;
// use std::time::Duration;
// use tokio::time;
// use uuid::Uuid;

// /// Only devices whose name contains this string will be tried.
// const PERIPHERAL_NAME_MATCH_FILTER: &str = "2301B";
// /// UUID of the characteristic for which we should subscribe to notifications.
// const NOTIFY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x6e400002_b534_f393_67a9_e50e24dccA9e);

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     pretty_env_logger::init();

//     let manager = Manager::new().await?;
//     let adapter_list = manager.adapters().await?;
//     if adapter_list.is_empty() {
//         eprintln!("No Bluetooth adapters found");
//     }

//     for adapter in adapter_list.iter() {
//         println!("Starting scan...");
//         adapter
//             .start_scan(ScanFilter::default())
//             .await
//             .expect("Can't scan BLE adapter for connected devices...");
//         time::sleep(Duration::from_secs(2)).await;
//         let peripherals = adapter.peripherals().await?;

//         if peripherals.is_empty() {
//             eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
//         } else {
//             // All peripheral devices in range.
//             for peripheral in peripherals.iter() {
//                 let properties = peripheral.properties().await?;
//                 let is_connected = peripheral.is_connected().await?;
//                 let local_name = properties
//                     .unwrap()
//                     .local_name
//                     .unwrap_or(String::from("(peripheral name unknown)"));
//                 println!(
//                     "Peripheral {:?} is connected: {:?}",
//                     &local_name, is_connected
//                 );
//                 // Check if it's the peripheral we want.
//                 if local_name.contains(PERIPHERAL_NAME_MATCH_FILTER) {
//                     println!("Found matching peripheral {:?}...", &local_name);
//                     if !is_connected {
//                         // Connect if we aren't already connected.
//                         if let Err(err) = peripheral.connect().await {
//                             eprintln!("Error connecting to peripheral, skipping: {}", err);
//                             continue;
//                         }
//                     }
//                     let is_connected = peripheral.is_connected().await?;
//                     println!(
//                         "Now connected ({:?}) to peripheral {:?}.",
//                         is_connected, &local_name
//                     );
//                     if is_connected {
//                         println!("Discover peripheral {:?} services...", local_name);
//                         peripheral.discover_services().await?;
//                         for characteristic in peripheral.characteristics() {
//                             println!("Checking characteristic {:?}", characteristic);
//                             // Subscribe to notifications from the characteristic with the selected
//                             // UUID.
//                             if characteristic.uuid == NOTIFY_CHARACTERISTIC_UUID
//                                 && characteristic.properties.contains(CharPropFlags::NOTIFY)
//                             {
//                                 println!("Subscribing to characteristic {:?}", characteristic.uuid);
//                                 peripheral.subscribe(&characteristic).await?;
//                                 // Print the first 4 notifications received.
//                                 let mut notification_stream =
//                                     peripheral.notifications().await?.take(4);
//                                 // Process while the BLE connection is not broken or stopped.
//                                 while let Some(data) = notification_stream.next().await {
//                                     println!(
//                                         "Received data from {:?} [{:?}]: {:?}",
//                                         local_name, data.uuid, data.value
//                                     );
//                                 }
//                             }
//                         }
//                         println!("Disconnecting from peripheral {:?}...", local_name);
//                         peripheral.disconnect().await?;
//                     }
//                 } else {
//                     println!("Skipping unknown peripheral {:?}", peripheral);
//                 }
//             }
//         }
//     }
//     Ok(())
// }
