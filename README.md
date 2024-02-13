# Gentoo 

### RESTful API Documentation

Gentoo is a lightweight HTTP request and response service to assist frontend devs in visualizing the structure of their requests. This documentation offers an overview of available endpoints and their functionality, catering to various HTTP methods and request types such as GET, POST, PUT, and DELETE.

## Endpoints

### GET /

This endpoint performs a health check on the Gentoo server.

#### Request

- Method: GET
- Path: /

#### Response

Upon successful execution, the server responds with an HTTP status code of 200 OK and a JSON object containing information about the Gentoo project, similar to the `/get` endpoint.

## Example Usage

### Test Endpoint



### Response Format

### GET /get

This endpoint retrieves a success response containing information about the Gentoo project.

#### Request

- Method: GET
- Path: /get

#### Response

Upon successful execution, the server responds with an HTTP status code of 200 OK and a JSON object containing information about the Gentoo project.

#### Response Body

The response body contains the following fields:

- `name`: The name of the Gentoo project.
- `about`: A brief description of the project.
- `created_by`: The name of the creator of the project.
- `message`: A message indicating the success of the request.
- `status_code`: The HTTP status code of the response.


## Endpoint: POST /post

This endpoint allows clients to send a POST request to the server.

### Request

- Method: POST
- Path: /post

#### Request Body

The request body should contain the data to be processed by the server.

#### Query Parameters

The query parameters should be included in the URL.

### Response

Upon successful execution, the server responds with an HTTP status code of 201 Created and a JSON object containing information about the request.

#### Response Body

The response body contains the following fields:

- `message`: A message indicating the success of the request.
- `status_code`: The HTTP status code of the response.
- `body`: The data received in the request body.
- `queries`: The query parameters received in the request URL.
- `headers`: The headers received in the request.

## Example Usage

### Request

```http
POST /post?id=123&message=test_message HTTP/1.1
Content-Type: application/json

{"key": "value"}

```



#### Response

```json
    {
    "message": "Request successfull",
    "status_code": "200",
    "body_data": "{\"id\":\"12345\",\"name\":\"User name\"}",
    "queries": {"id":123,"message":"test_message"},
    "headers": [["content-type","application/json"]]
    }
```


## Endpoint: PUT /put

This endpoint allows clients to send a PUT request to the server.

### Request

- Method: PUT
- Path: /put

#### Request Body

The request body should contain the data to be processed by the server.

#### Query Parameters

The query parameters should be included in the URL.

### Response

Upon successful execution, the server responds with an HTTP status code of 200 OK and a JSON object containing information about the request.

#### Response Body

The response body contains the following fields:

- `message`: A message indicating the success of the request.
- `status_code`: The HTTP status code of the response.
- `body`: The data received in the request body.
- `queries`: The query parameters received in the request URL.
- `headers`: The headers received in the request.

## Example Usage

### Request

```http
PUT /put?id=123&message=test_message HTTP/1.1
Content-Type: application/json

{"key": "value"}

```


### Response

```json
    {
        "message": "Request successful",
        "status_code": "200",
        "body": {
            "key": "value"
        },
        "queries": {
            "id": 123,
            "message": "test_message"
        },
        "headers": [
            {
            "name": "Content-Type",
            "value": "application/json"
            }
        ]
    }
```


## DELETE /delete

Mock endpoint to delete a resource.

### Request

- Method: DELETE
- Path: `/delete`
- Headers:
  - Content-Type: application/json
- Query Parameters:
  - id: The identifier of the resource (integer)
  - message: A message related to the request (string)
- Body: JSON data representing the resource to be deleted

Example Request:
```http
DELETE /delete?id=123&message=test_message HTTP/1.1
Content-Type: application/json

{"key": "value"}
```

### Response

- Status Code: 200 OK
- Headers: None
- message: A message indicating the success of the request
- status_code: The status code of the response (string)
- body_data: The data sent in the request body (JSON object)
- queries: The query parameters sent with the request (JSON object)
- headers: All the headers sent with the request (JSON object)

### Example Response:

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
        {
            "Custom-Header": "Custom Value"
        }
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
  
### Motivation

The project started as a way for us to mock endpoints, mostly for testing purposes with my friend @Gee. We wanted to inspect the structure of our requests, including headers, query parameters, and body fields. So, I decided to create this project, and I hope someone finds it helpful too. We still actively use it in our development workflow.






