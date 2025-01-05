# crates.io Downloads Plotting

The presented Rust code creates a plot as discussed in [this post](https://jpanda736.substack.com/p/benfords-law-in-cratesio-download) looking like the following one. Data as shown in the plot is derived from the crates.io database dumps. See [here](https://crates.io/data-access) for further details on the database dumps.

![Plot of leading digits resulting from running `cargo run` showing resemble to distribution of leading as stated by Benford's law.](plot.png "Resulting plot")

## Steps to recreate

1. Download database dumps via `curl https://static.crates.io/db-dump.tar.gz -L --output db-dump.tar.gz`.
2. Extract archive for example by using `tar -xvzf db-dump.tar.gz`.
3. Navigate to the data folder. Depending on the file name (as given by the download date) this might look something like this `cd 2025-01-04-020017`. 
4. Run database container `podman run --name crates-io-db-dumb -e POSTGRES_USER=user -e POSTGRES_PASSWORD=password -e POSTGRES_DB=db_dump -p 5432:5432 -v $(pwd):/data -d postgres`. In case the postgres image needs to be pulled prior this can be done via `podman pull docker.io/library/postgres`.
5. Execute import scripts for data `podman exec -it -w=/data crates-io-db-dumb psql -U user -d db_dump -f schema.sql && podman exec -it -w=/data crates-io-db-dumb psql -U user -d db_dump -f import.sql`. This might take some time as data needs to be copied to the tables.

After these steps there is a postgres database container running providing access to the crates.io data dump. Note that, you can as an alternative to podman can also use docker if favored. The commands should be compatible. The plot can be created executing `cargo run` in the projects main folder. After compiling and running there is a `plot.png` file stored in the folder.
