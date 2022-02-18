FROM debian

ENV HOME=/home/root
WORKDIR $HOME

# Required dependencies
RUN apt-get update \
    && apt-get install -y curl build-essential libssl-dev openssl pkg-config \
    && apt-get autoremove -y \
    && apt-get clean

# Cargo + Rust
ENV CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
RUN cargo install trunk
RUN rm -r /usr/local/cargo/registry

# Google Chrome
RUN curl https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add - \
    && echo "deb http://dl.google.com/linux/chrome/deb/ stable main" >> \
        /etc/apt/sources.list.d/google.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends google-chrome-stable

ENTRYPOINT []
CMD ["/bin/bash"]
