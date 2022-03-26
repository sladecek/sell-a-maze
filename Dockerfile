FROM python:3.7
RUN apt-get update && apt-get -y install ca-certificates libgmp3-dev
RUN pip install --no-cache-dir cairo-lang 
COPY web .
COPY static static
COPY work work
CMD ["/web"]
