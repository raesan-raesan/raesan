FROM ubuntu:24.10

RUN apt-get update && apt-get install -y \
    curl git gnupg unzip build-essential tmux \
    libssl-dev libreadline-dev zlib1g-dev libsqlite3-dev libbz2-dev libffi-dev liblzma-dev

RUN git clone https://github.com/asdf-vm/asdf.git ~/.asdf --branch v0.13.1

RUN echo '. ~/.asdf/asdf.sh' >> ~/.bashrc

COPY .tool-versions /root/

RUN bash -c 'source ~/.asdf/asdf.sh && \
    cat /root/.tool-versions | awk "{print \$1}" | xargs -I {} asdf plugin add {} && \
    asdf install'

WORKDIR /usr/src/app

COPY . .

RUN /root/.asdf/shims/cargo build
RUN /root/.asdf/shims/bun install --yarn

CMD ["tail", "-f", "/dev/null"]
