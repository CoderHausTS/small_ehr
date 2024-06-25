[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]


<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/CoderHausTS/smallEHR">
    <img src="images/small_logo_transparent.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">smallEHR</h3>
<h4 align="center">Empowering Technology Learning in Healthcare</h4>

  <p align="center">
    Limited purpose Electronic Health Record system. 
    <br />
    Used to explore ideas in healthcare technology, and as a teaching tool for those who would like to enter the healthcare tech space.
    <br />
    <br />
    <a href="https://github.com/CoderHausTS/smallEHR"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/CoderHausTS/smallEHR">View Demo</a>
    ·
    <a href="https://github.com/CoderHausTS/smallEHR/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>
    ·
    <a href="https://github.com/CoderHausTS/smallEHR/issues/new?labels=enhancement&template=feature-request---.md">Request Feature</a>
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

[![Product Name Screen Shot][product-screenshot]]


<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

* Rust
* Diesel
* Axum

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- GETTING STARTED -->
## Getting Started

To get a local copy up and running follow these simple example steps. (TBD)

### Prerequisites

* rust 
  * Install rust following the [official documentation](https://www.rust-lang.org/learn/get-started)
* postgreSQL client an library (libpq)
* mysqlclient (for Diesel CLI)
* sqlite3 (for Diesel CLI)
* diesel_cli crate - follow the docs at [https://diesel.rs/guides/getting-started](https://diesel.rs/guides/getting-started)

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/CoderHausTS/smallEHR.git
   ```
2. Install crates and get the project ready
   ```sh
   cargo build 
   ```
3. Spin up PostgreSQL docker container
   ```sh
   docker compose up
   ```
4. Run diesel setup and migrations
    ```sh
    diesel setup
    ```
    ```sh
    diesel migration run
    ```
5. Run environment script (maybe)
    ```sh
    . ./somescript.sh
    ```
6. Build and run
   ```sh
   cargo run -- run
   ```

### Post install
1. Import data
   Use synthetic data if you are testing. A good resource is [Mitre Synthea](https://synthea.mitre.org/downloads)
2. Use the data cleaning scripts in file_utils to clean and prep your data for the schema.
3. Either import via your fav CLI tool, or use the built in import commands such as
   ```sh
	cargo run -- import csv patients "file_location"
   ```
<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- USAGE EXAMPLES -->
## Usage

To run the server side, simply build and feed run to the executable, or build the project and do the same
    ```sh
    cargo run -- run
    ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- ROADMAP -->
## Roadmap

- [ ] Import/export data functionality
- [ ] RESTful APIs
- [ ] User/Roles
  - [ ] Authorization/authentication
- [ ] Front end
- [ ] FHIR integration 

See the [open issues](https://github.com/CoderHausTS/smallEHR/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Project Link: [https://github.com/CoderHausTS/smallEHR](https://github.com/CoderHausTS/smallEHR)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* []()
* []()
* []()

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/CoderHausTS/smallEHR.svg?style=for-the-badge
[contributors-url]: https://github.com/CoderHausTS/smallEHR/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/CoderHausTS/smallEHR.svg?style=for-the-badge
[forks-url]: https://github.com/CoderHausTS/smallEHR/network/members
[stars-shield]: https://img.shields.io/github/stars/CoderHausTS/smallEHR.svg?style=for-the-badge
[stars-url]: https://github.com/CoderHausTS/smallEHR/stargazers
[issues-shield]: https://img.shields.io/github/issues/CoderHausTS/smallEHR.svg?style=for-the-badge
[issues-url]: https://github.com/CoderHausTS/smallEHR/issues
[license-shield]: https://img.shields.io/github/license/CoderHausTS/smallEHR.svg?style=for-the-badge
[license-url]: https://github.com/CoderHausTS/smallEHR/blob/master/LICENSE.txt
[product-screenshot]: images/screenshot.png


