FROM postgres:16

ENV POSTGRES_DB='metw-v2'
ENV POSTGRES_USER='metwcc'
ENV POSTGRES_PASSWORD='insecure-password_NO-NOT-EXPOSE-THIS-DB-TO-PUBLIC'

ARG fixtures

COPY *.sql /docker-entrypoint-initdb.d/
COPY fixtures/ /tmp/fixtures

RUN if [ -n "$fixtures" ]; then \
        mv /tmp/fixtures/*.sql /docker-entrypoint-initdb.d/; \
    fi
