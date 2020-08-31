# Jake's personal blog

This is the source to my personal blog, built on top of [Hugo](http://gohugo.io). This will probably be a place for me to test out some front-end things as well as blog.

## Running the site
```
hugo server
```

This serves a development site at `localhost:1313`.

## Building the site
```
hugo
```

This generates all of the files needed for production and stores them in the `public/` directory.

## Docker

The Dockerfile included here rebuilds the website and creates an NGINX image ready to serve the site on the port specified by the PORT environment variable.

```
docker build -t jakestambaugh/jake.land:latest
docker run -it --rm -e PORT=9090 -p 9090:9090 --name jake jakestambaugh/jake.land:latest
```

Note the `-e PORT=9090` argument, this is required to bind to a port inside the container.