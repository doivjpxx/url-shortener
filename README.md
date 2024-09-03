# URL Shortener

This is a simple URL shortener app built with Rust. It allows users to shorten long URLs into shorter, more manageable links.

## Features

- Shorten long URLs into shorter links
- Redirect users to the original URL when they visit the shortened link
- Track the number of clicks on each shortened link

## Installation

Make sure you have Rust installed on your machine. If not, you can install it by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

Also, you need to have MySQL database installed on your machine. You can install it by following the instructions on the [official website](https://dev.mysql.com/doc/mysql-installation-excerpt/8.0/en/).

To run MySQL database, you can use the following command:

```bash
docker run --name mysql -e MYSQL_ROOT_PASSWORD=password -d -p 3306:3306 mysql:latest
```

Then, create a new database named `url_shortener`:

```bash
docker exec -it mysql mysql -uroot -ppassword -e "CREATE DATABASE url_shortener;"
```

After that, create a `.env` file in the project directory with the following content:

```env
DATABASE_URL=mysql://root:password@localhost/url_shortener
```

Follow the steps below to run the app:

1. Clone the repository: `git clone https://github.com/doivjpxx/url-shortener.git`
2. Navigate to the project directory: `cd url-shortener`
3. Build the project: `cargo build`
4. Run the app: `cargo run`

## Run with Dockerfile

1. Build the image: `docker build -t url-shortener .`
2. Run the container: `docker run -p 8000:8000 url-shortener`

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

