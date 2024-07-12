# AWS IoT to PostgreSQL Data Ingestion

This project demonstrates how to subscribe to an AWS IoT topic using MQTT, process incoming messages, and store them in a PostgreSQL database. The script connects to the AWS IoT Core, subscribes to a topic, and inserts the received data into an RDS PostgreSQL instance.

## Prerequisites

Before you begin, ensure you have the following:

- Python 3.x installed on your system
- Access to an AWS account with IoT Core and RDS PostgreSQL set up
- The following Python packages installed:
  - `paho-mqtt`
  - `psycopg2`
- AWS IoT Core endpoint, CA certificate, client certificate, and private key
- RDS PostgreSQL instance details (hostname, username, password, database name)

## Installation

1. Clone this repository or download the script to your local machine.
2. Install the required Python packages using pip:
    ```sh
    pip install paho-mqtt psycopg2
    ```

## Configuration

Update the script with your specific details:

1. **RDS PostgreSQL Connection Parameters:**
    ```python
    host = "your-rds-endpoint"
    username = "your-username"
    password = "your-password"
    database = "your-database-name"
    ```

2. **AWS IoT Core Certificates and Keys:**
    ```python
    ca_path = "path-to-your-CA-certificate"
    cert_path = "path-to-your-client-certificate"
    key_path = "path-to-your-private-key"
    ```

3. **AWS IoT Core Endpoint:**
    ```python
    client.connect("your-iot-endpoint", 8883, 60)
    ```

## Running the Script

1. Ensure your RDS PostgreSQL instance is running and accessible.
2. Run the script:
    ```sh
    python3 your-script-name.py
    ```
3. The script will connect to the AWS IoT Core, subscribe to the specified topic, and start listening for messages. When a message is received, it will be parsed and inserted into the PostgreSQL database.

## Code Explanation

The script consists of the following parts:

1. **Imports and Configuration:**
    - Import necessary libraries.
    - Define RDS PostgreSQL connection parameters.

2. **MQTT Callbacks:**
    - `on_connect`: Called when the MQTT client connects to the broker.
    - `on_message`: Called when a message is received from the subscribed topic. The message is decoded, parsed, and inserted into the PostgreSQL database.

3. **MQTT Client Setup:**
    - Create an MQTT client instance.
    - Set the on_connect and on_message callbacks.
    - Configure TLS/SSL settings with the CA certificate, client certificate, and private key.
    - Connect to the AWS IoT Core endpoint and start the loop to listen for messages indefinitely.

## Troubleshooting

If you encounter issues, check the following:

- Ensure your RDS PostgreSQL instance is accessible and the connection parameters are correct.
- Verify the AWS IoT Core certificates and keys are correct and the paths are accurate.
- Make sure the topic you are subscribing to in AWS IoT Core matches the one in the script.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Paho MQTT Python](https://www.eclipse.org/paho/index.php?page=clients/python/index.php)
- [psycopg2](https://www.psycopg.org/)






### output

```python
ravi@ravi:~/ECL2 Projects/project yashwanth/lambdadeploytest$ python3 test.py 
/home/ravi/ECL2 Projects/project yashwanth/lambdadeploytest/test.py:132: DeprecationWarning: Callback API version 1 is deprecated, update to latest version
  client = mqtt.Client()
Connected with result code 0
test1234 b'{"text":"Float(123.0)\\t"}'
Received message: {"text":"Float(123.0)\t"}
<class 'dict'>
Data inserted successfully.
test1234 b'{"text":"Float(123.0)\\t"}'
Received message: {"text":"Float(123.0)\t"}
<class 'dict'>
Data inserted successfully.

    ```



