# Enable BuildKit features
#syntax=docker/dockerfile:1.4

# Base stage with common dependencies
FROM debian:bookworm-slim as base

# Create omniforge user with proper home directory
RUN groupadd -g 1000 omniforge && \
    useradd -m -s /bin/bash -u 1000 -g omniforge omniforge && \
    mkdir -p /home/omniforge && \
    chown -R omniforge:omniforge /home/omniforge

RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update && export DEBIAN_FRONTEND=noninteractive && \
    apt-get install -y \
    curl \
    git \
    build-essential \
    pkg-config \
    libssl-dev \
    libdbus-1-dev \
    python3-full \
    python3-pip \
    python3-venv \
    clang \
    sudo \
    && rm -rf /var/lib/apt/lists/*

# Let's make sure that omniforge is a sudoer.
RUN usermod -aG sudo omniforge

# Also let's make sure that omniforge is not prompted for a password when using sudo because there are stages that
# use sudo in this build.
RUN echo "omniforge ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Rust stage
FROM base as rust-builder
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH=/usr/local/cargo/bin:$PATH

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    rustup default stable && \
    rustup component add rust-analyzer rust-src

# Install core Rust tools
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo install --root /usr/local/cargo-tools \
    minijinja-cli \
    cargo-edit \
    cargo-watch \
    cargo-binstall \
    cargo-nextest \
    taplo-cli \
    tokio-console && \
    cargo install --root /usr/local/cargo-tools --git https://github.com/helix-editor/helix helix-term

# Node.js stage
FROM base as node-builder
ENV NVM_DIR=/home/omniforge/.nvm
USER omniforge
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash && \
    . $NVM_DIR/nvm.sh && \
    nvm install --lts && \
    nvm use --lts && \
    nvm alias default lts/* && \
    npm install -g yarn bun && \
    echo 'export NVM_DIR="$HOME/.nvm"' >> ~/.bashrc && \
    echo '[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"' >> ~/.bashrc && \
    echo '[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"' >> ~/.bashrc && \
    echo 'export PATH="$HOME/.nvm/versions/node/$(ls $HOME/.nvm/versions/node)/bin:$PATH"' >> ~/.bashrc && \
    echo 'export PATH="$HOME/.nvm/versions/node/$(ls $HOME/.nvm/versions/node)/bin:$PATH"' >> ~/.zshrc

# Python stage
FROM base as python-builder
ENV VIRTUAL_ENV=/opt/venv
RUN python3 -m venv $VIRTUAL_ENV
ENV PATH="$VIRTUAL_ENV/bin:$PATH"

RUN pip install --no-cache-dir \
    ruff \
    black \
    mypy \
    pytest \
    pytest-cov \
    ipython \
    sqlfluff \
    pgcli

# Shell tools stage
#FROM base as shell-builder
#USER omniforge
#RUN sudo apt-get update && sudo apt-get install -y \
#    fish \
#    zsh \
#    && sudo rm -rf /var/lib/apt/lists/*
#
## Install shell configurations
#RUN curl -sS https://starship.rs/install.sh | sudo sh -s -- -y && \
#    bash -c "$(curl -fsSL https://raw.githubusercontent.com/ohmybash/oh-my-bash/master/tools/install.sh)" || true && \
#    sed -i 's/OSH_THEME=".*"/OSH_THEME="agnoster"/' ~/.bashrc && \
#    mkdir -p ~/.config/fish && \
#    curl -L https://raw.githubusercontent.com/oh-my-fish/oh-my-fish/master/bin/install > install && \
#    fish install --noninteractive && \
#    sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" || true && \
#    sed -i 's/ZSH_THEME=".*"/ZSH_THEME="agnoster"/' ~/.zshrc

# Final stage
FROM base as final

# If the user omniforge doesn't own these directories and the files within, then the command "cargo build" doesn't
# work properly in this environment.

# NOTE: The Google AI suggested that there is a problem with ownership when you use the Docker copy command and the
# destination doesn't already exist.  Supposedly in that scenario, Docker will create the directory using root
# regardless of the ownership specified when using the --chown switch on the COPY command.

# The suggested remedy is to create the directory first.  Not sure this actually needs to happen but in the latest
# build the ownership finally was working better.  Not sure if it's because of this remedy or if it's because I finally
# purged everything and started over.  Feel free to remove this in the future if you find it's not necessary.
RUN mkdir -p /usr/local/cargo && chown omniforge:omniforge /usr/local/cargo
RUN mkdir -p /usr/local/rustup && chown omniforge:omniforge /usr/local/rustup
RUN mkdir -p /usr/local/cargo-tools && chown omniforge:omniforge /usr/local/cargo-tools

# Copy from Rust stage
# NOTE: Also, added --chown switch to set the ownership to the omniforge user.
COPY --from=rust-builder --chown=omniforge:omniforge /usr/local/cargo /usr/local/cargo
COPY --from=rust-builder --chown=omniforge:omniforge /usr/local/rustup /usr/local/rustup
COPY --from=rust-builder --chown=omniforge:omniforge /usr/local/cargo-tools /usr/local/cargo-tools
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH=/usr/local/cargo/bin:/usr/local/cargo-tools/bin:$PATH

# Install additional Rust tools
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo install \
    aichat \
    bat \
    bottom \
    code2prompt \
    dua-cli \
    du-dust \
    exa \
    fd-find \
    hyperfine \
    procs \
    ripgrep \
    tealdeer \
    tokei \
    zoxide

# After the last build, found that there were still some files owned by root which were causing a problem with
# the command "cargo install just" executing properly.  This is a last ditch effort to force the ownership so
# the cargo command will work properly.
RUN chown -R omniforge:omniforge /usr/local/cargo
RUN chown -R omniforge:omniforge /usr/local/rustup
RUN chown -R omniforge:omniforge /usr/local/cargo-tools

USER omniforge
WORKDIR /home/omniforge

RUN cargo install just

# Copy from Node.js stage
COPY --from=node-builder --chown=omniforge:omniforge /home/omniforge/.nvm /home/omniforge/.nvm
COPY --from=node-builder --chown=omniforge:omniforge /home/omniforge/.bashrc /home/omniforge/.bashrc
COPY --from=node-builder --chown=omniforge:omniforge /home/omniforge/.zshrc /home/omniforge/.zshrc
ENV NVM_DIR=/home/omniforge/.nvm
ENV PATH="/home/omniforge/.nvm/versions/node/$(ls /home/omniforge/.nvm/versions/node)/bin:$PATH"
RUN . $NVM_DIR/nvm.sh && \
    nvm install --lts && \
    nvm use --lts && \
    nvm alias default lts/* && \
    npm install -g yarn bun

# Copy from Python stage
COPY --from=python-builder /opt/venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"
ENV VIRTUAL_ENV=/opt/venv

# Copy shell configurations
#COPY --from=shell-builder --chown=omniforge:omniforge /home/omniforge/.oh-my-bash /home/omniforge/.oh-my-bash
#COPY --from=shell-builder --chown=omniforge:omniforge /home/omniforge/.oh-my-zsh /home/omniforge/.oh-my-zsh
#COPY --from=shell-builder --chown=omniforge:omniforge /home/omniforge/.config/fish /home/omniforge/.config/fish
#COPY --from=shell-builder /usr/local/bin/starship /usr/local/bin/starship
#COPY --from=shell-builder --chown=omniforge:omniforge /home/omniforge/.bashrc /home/omniforge/.bashrc
#COPY --from=shell-builder --chown=omniforge:omniforge /home/omniforge/.zshrc /home/omniforge/.zshrc

# Install Nushell
#RUN sudo curl -LO https://github.com/nushell/nushell/releases/download/0.89.0/nu-0.89.0-x86_64-linux-gnu-full.tar.gz && \
#    sudo tar -xvzf nu-0.89.0-x86_64-linux-gnu-full.tar.gz && \
#    sudo cp nu-0.89.0-x86_64-linux-gnu-full/nu* /usr/local/bin/ && \
#    sudo rm -rf nu-0.89.0-x86_64-linux-gnu-full*

# Set default shell to zsh
ENV SHELL=/bin/bash
CMD ["bash"]
