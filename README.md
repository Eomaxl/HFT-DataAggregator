# Data Aggregator (Stock Exchange Data)
<p>
This application is designed to function as a real-time data aggregator and trading system, capable of collecting, 
processing, and distributing market data to facilitate automated trading decisions. 
It integrates multiple protocols such as FIX, WebSocket, and REST APIs to manage real-time data streams, execute trades,
and provide analytics. This system is ideal for use in financial markets where low latency and 
high concurrency are critical.
</p>

# Use Cases
<ul>
    <li>
        <b>Real-Time Market Data Aggregation:</b> 
        The application collects live market data using WebSocket and REST APIs, aggregates it,
        and makes it available for trading algorithms.
    </li>
    <li>
        <b>Automated Trading: </b>
        The FIX protocol is used for executing trades based on real-time data processed by the system.
    </li>
    <li>
        <b>Market Depth Analysis: </b>
        Fetches order book data to provide insights into market liquidity and potential price movements.
    </li>
</ul>

# APIs Exposed by the Application
<h4>1. Stock List API</h4>
<ul>
    <li>
        Endpoint: http://127.0.0.1:8080/stocks
    </li>
    <li>Method: GET</li>
    <li>Query Parameters (all optional):
        <ul>
            <li>symbol: Filter by symbol (e.g., AAPL).</li>
            <li>exchange: Filter by exchange name (e.g., NASDAQ).</li>
            <li>mic_code: Filter by market identifier code (MIC) (e.g., XNAS).</li>
            <li>country: Filter by country (e.g., United States).</li>
            <li>type_: Filter by instrument type (e.g., Common Stock).</li>
        </ul>
    </li>
    <li>Example curl Command:
        <ul>
            <li>curl "http://127.0.0.1:8080/stocks?symbol=AAPL&exchange=NASDAQ"</li>
        </ul>
    </li>
    <li>Example Response:
        <ul>
            <li>[
    {
        "symbol": "AAPL",
        "name": "Apple Inc",
        "currency": "USD",
        "exchange": "NASDAQ",
        "mic_code": "XNAS",
        "country": "United States",
        "type_": "Common Stock"
    }
]
</li>
        </ul>
    </li>
</ul>
# Building and Running the Application
<b>Prerequisites</b>
<ul>
    <li>
        <b>Rust :</b> Ensure that Rust is installed on your machine. You can install Rust by following the instructions at rust-lang.org.
    </li>
    <li>
        <b>Cargo :</b> Rust’s package manager, cargo, will be used to build and run the application.
    </li>
</ul>

<h6><b>Steps to Build and Run</b><br></h6>
<b>Clone the Repository: </b><br>
git clone https://github.com/your-repo/trading-system.git <br>
cd trading-system

<h6><b>Create a .env File: </b></h6>
<p>Create a .env file in the root of the project and add the following:</p>
API_KEY=your_twelve_data_api_key
FIX_SERVER_ADDRESS=127.0.0.1
FIX_SERVER_PORT=12345

<h6><b>Build the Application:</b></h6>
cargo build --release

<h6><b>Run the Application:</b></h6>
cargo run --release <br>
The server will start and listen on http://127.0.0.1:8080.

# Design Patterns Used
<h4>Actor Model:</h4>
<p>Used in WebSocket handling via the actix framework, where each WebSocket connection is managed as an actor.</p>

<h4>Factory Pattern:</h4>
<p>Employed in creating and managing instances of clients like the TwelveDataAPI and FixSession.</p>

<h4>Observer Pattern: </h4>
<p> The broadcast channel in WebSocket and FIX sessions implements an observer pattern where multiple subscribers can receive updates.</p>

# Protocols Overview
WebSocket<br>
<p><b>Purpose:</b> WebSockets provide full-duplex communication channels over a single TCP connection. In this application, WebSockets are used to receive real-time updates from market data providers and push them to connected clients.</p>
FIX (Financial Information eXchange)<br>
<p><b>Purpose:</b> FIX is a standardized protocol used for real-time electronic communication of financial transactions. The application uses FIX for executing trades and managing orders.</p>
REST<br>
<p><b>Purpose:</b> REST (Representational State Transfer) is an architectural style for designing networked applications. It uses HTTP requests to access and manipulate data.</p>