AWS Lambda Function to Fetch Data from PostgreSQL

This project contains an AWS Lambda function written in Python that connects to a PostgreSQL database hosted on Amazon RDS, retrieves data from the sensor table, and returns the data in JSON format. The Lambda function is designed to be invoked via an API Gateway endpoint.
Table of Contents

    Prerequisites
    Installation
    Deployment
    Usage
    Local Testing
    Error Handling
    Acknowledgments

Prerequisites

    AWS Account
    Amazon RDS PostgreSQL instance
    AWS Lambda function
    AWS API Gateway
    Python 3.8 or higher
    psycopg2 library (included in Lambda deployment package)

Installation

    Set up your PostgreSQL database on Amazon RDS:

    Ensure your PostgreSQL instance is running and accessible. Note down the endpoint, username, and password.

    Create the sensor table in your PostgreSQL database:

    sql

    CREATE TABLE public.sensor (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        value VARCHAR(255) NOT NULL,
        timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

    Create an AWS Lambda function:

    Create a new Lambda function in the AWS Lambda console.

    Install the psycopg2 library:

    Lambda requires the psycopg2 library to interact with PostgreSQL. You can include it in your deployment package.

Deployment

    Prepare your deployment package:

    Create a deployment package that includes your Lambda function code and the psycopg2 library.

        Directory structure:

    deployment_package/
    ├── lambda_function.py
    ├── psycopg2/
    └── psycopg2_binary/

    Create a ZIP file of the deployment_package directory.

bash

    zip -r deployment_package.zip deployment_package

    Upload the deployment package:

    Upload the deployment_package.zip to your Lambda function in the AWS Lambda console.

    Set environment variables:

    Configure the following environment variables for your Lambda function:
        HOST: Your PostgreSQL RDS endpoint
        USERNAME: Your PostgreSQL username
        PASSWORD: Your PostgreSQL password
        DATABASE: Your PostgreSQL database name

Usage

    Set up API Gateway:
        Create a new API in API Gateway.
        Create a resource (e.g., /sensor).
        Create a GET method for the resource.
        Integrate the GET method with your Lambda function.
        Deploy the API to a stage (e.g., dev).

    Invoke the API:

    Send a GET request to your API endpoint to retrieve the data from the sensor table.

    Example endpoint: https://your-api-id.execute-api.us-east-1.amazonaws.com/dev/sensor

Local Testing

For local testing, you can mock the event and context and invoke the lambda_handler function directly.

Uncomment the local testing code in lambda_function.py and run the script:

python

# Mock event and context for local testing
if __name__ == "__main__":
    dummy_event = {}
    dummy_context = {}
    response = lambda_handler(dummy_event, dummy_context)
    print(response)

Error Handling

The Lambda function includes basic error handling for database connection issues and query execution errors. In case of an error, the function returns a 500 status code with an error message.
Acknowledgments

    This project uses the psycopg2 library for PostgreSQL database interaction.
    The AWS Lambda and API Gateway documentation were used as references.




#output

```python
ravi@ravi:~/ECL2 Projects/project yashwanth/fetchfromdatabase$ python3 fetch.py 
{'statusCode': 200, 'body': '[{"value": "Float(123.0)\\t"}, {"value": "Float(123.0)\\t"}, {"value": "Float(123.0)\\t"}, {"value": "Float(123.0)\\t"}, {"value": "Float(123.0)\\t"}, {"value": "Float(123.0)\\t"}, {"value": "Float(123.0)\\t"}, {"value": "Float(123.0)\\t"}, {"value": "Float(123.0)\\t"}, {"value": "Float(123.0)\\t"}, {"value": "Float(123.0)\\t"}, {"value": "Float(123.0)\\t"}]', 'headers': {'Content-Type': 'application/json', 'Access-Control-Allow-Origin': '*'}}

    ```