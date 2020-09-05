---
title: "Up and Running With Hugo"
date: 2020-09-04T14:20:28-04:00
---
After kludging together a few HTML pages and a [Rocket](http://rocket.rs) server to make a "blog" last month, I started to run into issues with templating. It wasn't that creating a template was difficult in Rocket; the problem was that I was using it for the wrong think (kinda). 

My goal was to write blog posts in Markdown, run them through a Markdown parser that could generate HTML, and then splice them into the Handlebars templates using Rocket's templating engine. This had two downsides: I would have to write all of the Markdown file handling code myself, and I would be taking time during each request to splice the converted Markdown into the Handlebars template. I realized that I didn't need to regenerate the templates during each request, since every user was going to be viewing the same page.

Instead of rebuilding my Rocket server to serve "static pages" I looked for a static site generator. Static site generators do exactly what I was looking for. They generator HTML files from a set of templates and Markdown files, but they don't worry about serving them.

Not knowing much, I searched around and found [Hugo](http://gohugo.io). I didn't do much comparison shopping, but [Jekyll](http://jekyllrb.com) and [Zola](http://getzola.org) seemed to work almost exactly the same. At the end of the day, the important part is the Markdown files with the blog posts, and they seem almost 100% interchangeable between these systems.

The most interesting thing that I added to the project was a Dockerfile that would rebuild and serve the site through [NGINX](http://nginx.com) when the container is run.

```docker
FROM alpine as build

WORKDIR /build
ENV HUGO_VERSION 0.74.3
ENV HUGO_BINARY hugo_${HUGO_VERSION}_Linux-64bit.tar.gz
RUN set -x && \
  apk add --update wget ca-certificates imagemagick && \
  wget https://github.com/gohugoio/hugo/releases/download/v${HUGO_VERSION}/${HUGO_BINARY} && \
  tar xzf ${HUGO_BINARY} && \
  mv hugo /usr/bin
COPY . .
RUN /usr/bin/hugo

# Use a nginx Docker image
FROM nginx
# Copy the static HTMLs to the nginx directory
COPY --from=build /build/public /usr/share/nginx/html
# Copy the nginx configuration template to the nginx config directory
COPY nginx/default.template /etc/nginx/conf.d/default.template
# Substitute the environment variables and generate the final config
CMD envsubst < /etc/nginx/conf.d/default.template > /etc/nginx/conf.d/default.conf && exec nginx -g 'daemon off;'
```

## What comes next?

Moving to Hugo, I was encouraged by some tutorials to use a "theme" instead of worrying about my own layout. I found a nice, minimal theme to start with, but I still want to use this blog as a way to learn more web technologies. I also want to make my blog stand out a little bit, and a pre-baked theme isn't going to do it. This means I'll have to get acquainted with Hugo themes at some point.