cargo build  --release
cp target/release/web .
docker build -t maze .
rm web

# docker build -t samic .
# docker run -p 2705:2705 samic
