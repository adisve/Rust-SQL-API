<div id="top"></div>


<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/adisve/Rust-SQL-API.git">
    <img src="https://github.com/adisve/Rust-SQL-API/blob/main/rust-logo.svg" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">Rust-SQL-API</h3>

  <p align="center">
    <br />
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

This is a simple terminal application written in the programming language Rust, together with the Diesel framework, which was used when creating the ORM.
The program connects to the user 'API' on the network 'localhost', with the password 'P@sSw0r_D' to the database 'pillow_db'.
The purpose of this application is to allow access to my local database for the project in the course Database Technique at Kristianstad University.
<p align="right">(<a href="#top">back to top</a>)</p>



### Built With

* [Diesel](https://diesel.rs/)
* [Rust](https://www.rust-lang.org/)

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

Currently this is only a debug version, but you can still run the program by manually
compiling it and running it.

### Prerequisites

These are the necessary libraries, binaries and general requirements
* rustc
  - ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``` on UNIX (requires curl package manager)
  - ```choco install rust``` on Windows (requires chocolatey package manager)

* [MySQL Client Library](https://dev.mysql.com/downloads/c-api/)

* [MySQL](https://www.mysql.com/downloads/)

### Installation

1. Get the SQL ddl for the database at [Database-SQL](https://github.com/adisve/Rust-SQL-API/blob/main/DATABASE.sql) and
   run it in your database. A user called 'API' with the password 'P@sSw0r_D' is also required.
   
2. Create a new user called "API" as root user in your MySQL server.
    ```sh
    CREATE USER 'API'@'localhost' IDENTIFIED BY 'P@sSw0r_D;'
    ```
    Now grant all privileges to user
    ```sh
    GRANT ALL PRIVILEGES ON pillow_db TO 'API'@'localhost'
    ```
   
2. Clone the repo
   ```sh
   git clone https://github.com/adisve/Rust-SQL-API.git
   ```
   
3. Cd into directory
   ```sh
   cd Rust-SQL-API
   ```
   
4. Build the project with cargo
   ```rs
   cargo build
   ```
   
5. Run the project
   ```rs
   cargo run
   ```


<p align="right">(<a href="#top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
<!--
## Usage

Use this space to show useful examples of how a project can be used. Additional screenshots, code examples and demos work well in this space. You may also link to more resources.

_For more examples, please refer to the [Documentation](https://example.com)_

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Adis Veletanlic - [@linkedin](https://www.linkedin.com/in/adis-veletanlic-2b51b4229/) - adis.veletanlic@gmail.com

Project Link: [https://github.com/adisve/Rust-SQL-API](https://github.com/adisve/Rust-SQL-API)

<p align="right">(<a href="#top">back to top</a>)</p>
