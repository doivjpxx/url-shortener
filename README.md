# URL Shortener

This is a simple URL shortener app built with Rust. It allows users to shorten long URLs into shorter, more manageable links.

## Features

- Shorten long URLs into shorter links
- Redirect users to the original URL when they visit the shortened link
- Track the number of clicks on each shortened link

## Installation

1. Clone the repository: `git clone https://github.com/doivjpxx/url-shortener.git`
2. Navigate to the project directory: `cd url-shortener`
3. Build the project: `cargo build`
4. Run the app: `cargo run`

## Usage

To shorten a URL, send a POST request to the `/shorten` endpoint with the following JSON payload:

```json
{
    "url": "https://www.example.com/very-long-url"
}
```

Retrieve the shortened URL by sending a GET request to the `/shorten/{id}` endpoint, where `{id}` is the unique identifier of the shortened URL.

With every retrieval of the shortened URL, the number of access will be incremented.

Update the url in the `shorten` endpoint to the url you want to shorten. You can update via PUT method to the `shorten/{id}` endpoint.

To get the statistics of the shortened URL, send a GET request to the `/shorten/{id}/stats` endpoint.

To delete a shortened URL, send a DELETE request to the `/shorten/{id}` endpoint.

The API will respond with a shortened URL that can be used to redirect to the original URL.

To visit a shortened URL, simply enter it in your browser's address bar.

## License

This project is licensed under the MIT License.

---
Roadmap from [https://roadmap.sh/projects/url-shortening-service](https://roadmap.sh/projects/url-shortening-service)

