docker build -t rust-shell .

docker run -ti -v "${PWD}":/code rust-shell bash
