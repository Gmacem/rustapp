FROM rust:slim-buster

RUN apt-get -y update && apt-get install -y python3 make build-essential libssl-dev zlib1g-dev libbz2-dev \
    libreadline-dev libsqlite3-dev wget curl llvm libncurses5-dev libncursesw5-dev \
    xz-utils tk-dev libffi-dev liblzma-dev python-openssl git python3-pytest pip3
RUN pip3 install pytest
RUN mkdir -p /tmp /app/rustapp
WORKDIR /app/rustapp

COPY . ./
RUN cargo build

CMD [ "pytest", "tests" ]
