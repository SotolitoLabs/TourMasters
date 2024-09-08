# TourMasters
Application for managing tours

## Dependencies

podman, make, rust, cargo, libpq

## Development

### Run it

```
make run
```

### Test

* Add a venue

```bash
$ curl -X POST -H 'Content-type: application/json' -d @tests/data/new_venue.json http://127.0.0.1:8000/venues/add
```

* List venues

```bash
$ curl http://127.0.0.1:8000/venues

```

* Get a static file

Static files are located in the `www/static` and are served by the `/public` API endpoint.
If you wish to serve a text, html, image, css or any static item just copy it to the `www/static` directory.

```bash
$ curl http://127.0.0.1:8000/public/test.txt
``

