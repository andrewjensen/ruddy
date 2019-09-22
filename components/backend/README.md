# Ruddy - Backend

```bash
cargo run --bin controller

cargo run --bin worker
```


## Example data flow

- [x] Initial state:

```json
{
  "jobs": []
}
```

- [ ] Command received, starting:

```json
{
  "jobs": [
    {
      "id": "fd6a5152-a831-4294-bda1-212fd1baa8b7",
      "status": "INITIALIZING",
      "worker_host": null
    }
  ]
}
```

- [ ] Worker initialized, ready for work:

```json
{
  "jobs": [
    {
      "id": "fd6a5152-a831-4294-bda1-212fd1baa8b7",
      "status": "READY",
      "worker_host": "http://localhost:3200"
    }
  ]
}
```

- [ ] Started a render job:

```json
{
  "jobs": [
    {
      "id": "fd6a5152-a831-4294-bda1-212fd1baa8b7",
      "status": "READY",
      "worker_host": "http://localhost:3200",
      "render": {
        "frame_start": 0,
        "frame_end": 3,
        "frames_to_render": 4,
        "frames_rendered": 2,
        "latest_frame_rendered": 1,
        "average_render_time": 3105.6
      }
    }
  ]
}
```

- [ ] Finished rendering, packaging assets:

```json
{
  "jobs": [
    {
      "id": "fd6a5152-a831-4294-bda1-212fd1baa8b7",
      "status": "READY",
      "worker_host": "http://localhost:3200",
      "render": {
        "frame_start": 0,
        "frame_end": 3,
        "frames_to_render": 4,
        "frames_rendered": 4,
        "latest_frame_rendered": 3,
        "average_render_time": 3208.1
      }
    }
  ]
}
```

- [ ] Finished packaging, sending:

```json
{
  "jobs": [
    {
      "id": "fd6a5152-a831-4294-bda1-212fd1baa8b7",
      "status": "READY",
      "worker_host": "http://localhost:3200",
      "render": {
        "frame_start": 0,
        "frame_end": 3,
        "frames_to_render": 4,
        "frames_rendered": 4,
        "latest_frame_rendered": 3,
        "average_render_time": 3208.1
      }
    }
  ]
}
```

- [ ] Finished sending, ready to download on static URL:

```json
{
  "jobs": [
    {
      "id": "fd6a5152-a831-4294-bda1-212fd1baa8b7",
      "status": "READY",
      "worker_host": "http://localhost:3200",
      "render": {
        "frame_start": 0,
        "frame_end": 3,
        "frames_to_render": 4,
        "frames_rendered": 4,
        "latest_frame_rendered": 3,
        "average_render_time": 3208.1
      },
      "asset": {
        "filename": "cubes-0000-0003.zip",
        "url": "https://example.com/assets/cubes-0000-0003.zip"
      }
    }
  ]
}
```
