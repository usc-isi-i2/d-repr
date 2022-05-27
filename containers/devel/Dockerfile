FROM ubuntu:16.04

RUN apt-get update -y && \
    apt-get install -y wget vim curl build-essential libssl-dev pkg-config cmake zlib1g-dev

# setup environment variables for Anaconda3 and Rust
ENV ANACONDA_HOME /root/anaconda3
ENV PATH /root/anaconda3/bin:/root/.cargo/bin:$PATH

# install anaconda 3
RUN cd /tmp && \
    wget https://repo.anaconda.com/archive/Anaconda3-2019.03-Linux-x86_64.sh && \
    bash Anaconda3-2019.03-Linux-x86_64.sh -b && \
    echo ANACONDA_HOME=/root/anaconda3/ >> /root/.bashrc && \
    echo PATH=/root/anaconda3/bin:\$PATH >> /root/.bashrc && \
    rm Anaconda3-2019.03-Linux-x86_64.sh && \
    conda install gxx_linux-64

# install rust & tarpaulin for code coverage
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    cargo install cargo-tarpaulin

# pre-build drepr to save time
WORKDIR /drepr
COPY Cargo.toml Cargo.lock /drepr/

#RUN cd /drepr && \
#    USER=builder cargo new abc && \
#    mv abc/src . && \
#    rm -rf abc && \
#    cargo build
#
## install extra dependencies
#ADD coverage.sh /usr/bin/coverage
#
#RUN pip install pycobertura && \
#    chmod +x /usr/bin/coverage

RUN apt-get -y update && apt-get -y install \
    libhdf5-serial-dev \
    netcdf-bin \
    libnetcdf-dev \