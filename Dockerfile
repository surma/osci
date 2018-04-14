FROM selenium/node-chrome:latest

USER root

ENV NVM_DIR /usr/local/nvm
ENV CHROME_BIN /opt/google/chrome/chrome
ENV INSIDE_DOCKER=1
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH

RUN apt-get update -qqy \
  && apt-get install -qy curl build-essential \
  && rm -rf /var/lib/apt/lists/* /var/cache/apt/* \
  && rm /bin/sh && ln -s /bin/bash /bin/sh \
  && chown seluser /usr/local

RUN wget -qO- https://raw.githubusercontent.com/creationix/nvm/v0.33.2/install.sh | bash \
  && source $NVM_DIR/nvm.sh \
  && nvm install v8

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path \
  && rustup target add wasm32-unknown-unknown

ADD tests/run_tests.sh /opt/run_tests.sh

CMD /opt/run_tests.sh
WORKDIR /usr/src
