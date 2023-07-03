# DVC Viewer

Creates a GUI to view DVC registered datasets.

[DVC](https://dvc.org/) is a version control system that specializes in tracking datasets and machine learning experiments. DVC requires knowledge of git and bash. Sometimes people who work with managing datasets may not be comfortable using these technologies. This project aims to be an easy to use platform where anywhere can view current registered datasets in DVC dataset registry repository.

## How to Run

Easiest way to run the application is to have docker installed on your computer

1) `docker compose build`

2) `docker compose up`

This will create two containers. A frontend web app and a backend REST api.

to view the website go to `localhost:3000`

### How to View Current DVC Datasets

On the main page of the app in the navigation bar's top right corner there a `view datasets` link you can click on. You can also go directly to `http://localhost:3000/dataset_viewer`

This page takes in a link to your DVC git repo. An example input would be `https://github.com/iterative/dataset-registry.git`

Hit the button to scan the repo for existing datasets. You will see data for any `.dvc` file that is registered in the repo. This can be used by people to quickly view the datasets they have without having to use git or dvc commands.

## Setup Dev Env

This application uses `next.js` for the frontend and `rust` for the backend REST api.

### Setup Frontend

1) clone the repository

2) `cd dvc-viewer`

3) `npm install`

4) `npm run dev`

### Setup Backend

1) install [Rust](https://www.rust-lang.org/tools/install)

2) `cd api`

3) `cargo build`

4) `cargo run`

## Features to be Added

- [x] View Current Datasets
- [ ] Add to Existing Dataset

I am open to other feature requests. This application is simple and is really meant to help aid those who may not be developers in keeping track of datasets. This project is meant to be community driven so please contribute.