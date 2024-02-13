# req-res

### RESTful API Documentation

req-res is a lightweight HTTP request and response service to assist frontend devs in visualizing the structure of their requests. This documentation offers an overview of available endpoints and their functionality, catering to various HTTP methods and request types such as GET, POST, PUT, and DELETE.

## Endpoints

### GET /health

Perform a health check on the Gentoo server.

#### Request

- URL: [https://mock-api-stkk.onrender.com](https://mock-api-stkk.onrender.com)
- Method: GET

#### Response

- Status Code: 200 OK
- Body: JSON object containing information about the Gentoo project

### GET /get

Retrieve information about the Gentoo project.

#### Request

- URL: [https://mock-api-stkk.onrender.com/get](https://mock-api-stkk.onrender.com/get)
- Method: GET

#### Response

- Status Code: 200 OK
- Body: JSON object containing information about the Gentoo project

### POST /post

Send a POST request to the server.

#### Request

- URL: [https://mock-api-stkk.onrender.com/post](https://mock-api-stkk.onrender.com/post)
- Method: POST
- Body: Data to be processed by the server

#### Response

- Status Code: 201 Created
- Body: JSON object containing information about the request

### PUT /put

Send a PUT request to the server.

#### Request

- URL: [https://mock-api-stkk.onrender.com/put](https://mock-api-stkk.onrender.com/put)
- Method: PUT
- Body: Data to be processed by the server

#### Response

- Status Code: 200 OK
- Body: JSON object containing information about the request

### DELETE /delete

Delete a resource.

#### Request

- URL: [https://mock-api-stkk.onrender.com/delete](https://mock-api-stkk.onrender.com/delete)
- Method: DELETE
- Headers:
  - Content-Type: application/json
- Query Parameters:
  - id: integer (identifier of the resource)
  - message: string (message related to the request)
- Body: JSON data representing the resource to be deleted

#### Response

- Status Code: 200 OK
- Body: JSON object containing information about the request

```json
    {
    "message": "Request successfull",
    "status_code": "200",
    "body_data": {
        "key": "value"
    },
    "queries": {
        "id": 123,
        "message": "test_message"
    },
    "headers": [
        {
            "Content-Type": "application/json"
        },
        {
            "Host": "example.com"
        },
        {
            "User-Agent": "<request user agent header>"
        },
        {
            "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"
        },
        // { ... all headers in the request `}
    ]
    }
    
```


# Contribution Guidelines

Thank you for considering contributing to this project! We welcome contributions from the community to help improve the project and make it better for everyone.

## How to Contribute

1. Fork the repository to your GitHub account.
2. Clone the forked repository to your local machine.
3. Create a new branch for your contribution:


## Contribution Types

We welcome contributions in the following areas:
- Security disclosures
- Bug fixes
- Feature enhancements
- Documentation improvements
- Code optimizations
- Performance improvements
- Tests

# TODO

- Mock Data Generation: Provide tools to generate realistic mock data for responses, including random data generation, predefined templates, and support for complex data structures.

- Rate Limiting and Throttling: Add support for rate limiting and throttling to prevent abuse and ensure fair usage of the service.

### Motivation

The project started as a way for us to mock endpoints, mostly for testing purposes with my friend @Gee. We wanted to inspect the structure of our requests, including headers, query parameters, and body fields. So, I decided to create this project, and I hope someone finds it helpful too. We still actively use it in our development workflow.






