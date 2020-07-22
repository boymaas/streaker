FROM node
MAINTAINER Boy Maas <boy.maas@gmail.com>

RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    ca-certificates curl file \
    build-essential \
    autoconf automake autotools-dev libtool xutils-dev && \
    rm -rf /var/lib/apt/lists/*

ENV SSL_VERSION=1.0.2k

RUN curl https://www.openssl.org/source/openssl-$SSL_VERSION.tar.gz -O && \
    tar -xzf openssl-$SSL_VERSION.tar.gz && \
    cd openssl-$SSL_VERSION && ./config && make depend && make install && \
    cd .. && rm -rf openssl-$SSL_VERSION*

ENV OPENSSL_LIB_DIR=/usr/local/ssl/lib \
    OPENSSL_INCLUDE_DIR=/usr/local/ssl/include \
    OPENSSL_STATIC=1

RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain nightly -y

ENV PATH=/root/.cargo/bin:$PATH
ENV USER root
RUN npm install -g aerobatic-cli

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install kubectl
RUN apt-get update && \
    apt-get install -y apt-transport-https gnupg2 && \
    rm -rf /var/lib/apt/lists/*
RUN curl -s https://packages.cloud.google.com/apt/doc/apt-key.gpg | apt-key add -
RUN echo "deb https://apt.kubernetes.io/ kubernetes-xenial main" | tee -a /etc/apt/sources.list.d/kubernetes.list
RUN apt-get update && \
    apt-get install -y kubectl && \
    rm -rf /var/lib/apt/lists/*

# Install musl-gcc
RUN apt-get update && \
    apt-get install -y musl musl-dev musl-tools && \
    rm -rf /var/lib/apt/lists/*


# Install docker cli
ENV DOCKERVERSION=19.03.9
RUN curl -fsSLO https://download.docker.com/linux/static/stable/x86_64/docker-${DOCKERVERSION}.tgz \
  && tar xzvf docker-${DOCKERVERSION}.tgz --strip 1 \
                 -C /usr/local/bin docker/docker \
  && rm docker-${DOCKERVERSION}.tgz

# We need this to build a static binary
RUN rustup target add x86_64-unknown-linux-musl

# We need the AWS CLI to login to docket and kubectl
#
WORKDIR /install
RUN curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
RUN unzip awscliv2.zip
RUN aws/install

# Small tool to enable subst of variables
# envsubst
RUN apt-get update && \
    apt-get install -y  gettext-base && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /source

CMD ["bash"]
