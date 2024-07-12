import json
import psycopg2
import paho.mqtt.client as mqtt

# RDS PostgreSQL connection parameters
host = "********************************************"
username = "********************************************"
password = "********************************************"
database = "********************************************"

def on_connect(client, userdata, flags, rc):
    print("Connected with result code " + str(rc))
    client.subscribe("********************************************")

def on_message(client, userdata, msg):
    print(msg.topic + " " + str(msg.payload))

    message = msg.payload.decode()  # Decode bytes to string
    print(f"Received message: {message}")

    iot_message = json.loads(msg.payload)
    print(type(iot_message))
    
    value = iot_message.get('text')
    
    if value is None:
        print("Missing required fields in the IoT message.")
        return
    
    try:
        conn = psycopg2.connect(
            host=host,
            database=database,
            user=username,
            password=password
        )
        cur = conn.cursor()
        
        insert_query = """
        INSERT INTO sensor (value) 
        VALUES (%s)
        """
        cur.execute(insert_query, (value,))

        conn.commit()
        cur.close()
        conn.close()
        print("Data inserted successfully.")
    except Exception as e:
        print(f"Failed to connect to the database: {e}")

client = mqtt.Client()
client.on_connect = on_connect
client.on_message = on_message

# Set the path to your CA certificate, client certificate, and private key
ca_path = "********************************************"
cert_path = "********************************************"
key_path = "********************************************"

client.tls_set(ca_certs=ca_path, certfile=cert_path, keyfile=key_path)

# Replace 'your-iot-endpoint.amazonaws.com' with your actual AWS IoT endpoint
client.connect("********************************************", 8883, 60)
client.loop_forever()



