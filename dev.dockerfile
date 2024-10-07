FROM ubuntu:24.10

ENV PATH="~/.local/bin:$PATH"
ENV TMUX_CONF="/usr/src/app/.tmux.conf"

RUN apt-get update && apt-get install -y \
    curl git gnupg unzip build-essential tmux psmisc \
    libssl-dev libreadline-dev zlib1g-dev libsqlite3-dev libbz2-dev libffi-dev liblzma-dev

RUN git clone https://github.com/asdf-vm/asdf.git ~/.asdf --branch v0.13.1

RUN echo '. ~/.asdf/asdf.sh && tmux() { if [ -n "$TMUX_CONF" ]; then command tmux -f "$TMUX_CONF" "$@"; else command tmux "$@"; fi; }' >> ~/.bashrc

COPY .tool-versions /root/

RUN bash -c 'source ~/.asdf/asdf.sh && \
    cat /root/.tool-versions | awk "{print \$1}" | xargs -I {} asdf plugin add {} && \
    asdf install'
RUN bash -c 'sh -c "$(curl --location https://taskfile.dev/install.sh)" -- -d -b ~/.local/bin'
RUN bash -c "curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh"

WORKDIR /usr/src/app

COPY . .

RUN /root/.asdf/shims/cargo build --workspace

RUN cd ./raesan && /root/.asdf/shims/bun install --yarn && cd ../raesan-dbm && /root/.asdf/shims/bun install --yarn

CMD ["tail", "-f", "/dev/null"]
