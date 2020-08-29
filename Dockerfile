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