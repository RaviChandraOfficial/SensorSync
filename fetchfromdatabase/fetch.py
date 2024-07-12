import json
import psycopg2
from psycopg2.extras import RealDictCursor

# RDS PostgreSQL connection parameters
host = "********************************************************************"
username = "********************************************************************"
password = "********************************************************************"
database = "********************************************************************"

def lambda_handler(event, context):
    try:
        conn = psycopg2.connect(
            host=host,
            database=database,
            user=username,
            password=password
        )
        cur = conn.cursor(cursor_factory=RealDictCursor)

        # Example query
        query = "SELECT * FROM public.sensor"
        cur.execute(query)
        rows = cur.fetchall()

        # Convert rows to a list of dictionaries
        result = [dict(row) for row in rows]

        cur.close()
        conn.close()

        return {
            'statusCode': 200,
            'body': json.dumps(result),
            'headers': {
                'Content-Type': 'application/json',
                'Access-Control-Allow-Origin': '*'
            }
        }
    except Exception as e:
        return {
            'statusCode': 500,
            'body': json.dumps({'error': str(e)}),
            'headers': {
                'Content-Type': 'application/json',
                'Access-Control-Allow-Origin': '*'
            }
        }

# # for local testing
# if __name__ == "__main__":
#     dummy_event = {}
#     dummy_context = {}
#     response = lambda_handler(dummy_event, dummy_context)
#     print(response)



