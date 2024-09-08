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
