FROM debian:bookworm-slim

ARG OPENCODE_PROFILE_FILE=auth.local.json
ARG OPENCODE_CONFIG_FILE=config.local.json

RUN apt-get update \
  && apt-get install -y --no-install-recommends \
    bash \
    build-essential \
    ca-certificates \
    curl \
    file \
    gh \
    git \
    nodejs \
    npm \
    openssh-client \
    procps \
    tar \
  && rm -rf /var/lib/apt/lists/*

ENV NONINTERACTIVE=1
ENV CI=1
ENV PATH="/home/linuxbrew/.linuxbrew/bin:/home/linuxbrew/.linuxbrew/sbin:${PATH}"

RUN useradd -m -s /bin/bash linuxbrew && mkdir -p /home/linuxbrew/.linuxbrew && chown -R linuxbrew:linuxbrew /home/linuxbrew
USER linuxbrew
RUN bash -lc "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
RUN /home/linuxbrew/.linuxbrew/bin/brew install anomalyco/tap/opencode
USER root
RUN ln -sf /home/linuxbrew/.linuxbrew/bin/opencode /usr/local/bin/opencode

ENV HOME=/root
ENV XDG_DATA_HOME=/root/.local/share
ENV XDG_CONFIG_HOME=/root/.config
ENV XDG_CACHE_HOME=/root/.cache

RUN mkdir -p /root/.local/share/opencode /root/.config/gh /root/.config/opencode /root/.cache/opencode /root/.ssh/keys /workspace
COPY meta/docker/auth/${OPENCODE_PROFILE_FILE} /root/.local/share/opencode/auth.json
COPY meta/docker/config/${OPENCODE_CONFIG_FILE} /root/.config/opencode/opencode.json
COPY meta/docker/opencode/AGENTS.local.md /root/.config/opencode/AGENTS.md
COPY meta/docker/cache/models.local.json /root/.cache/opencode/models.json
COPY meta/docker/gh/hosts.local.yml /root/.config/gh/hosts.yml
COPY meta/docker/gh/token.local /tmp/gh-token
COPY meta/docker/ssh/config.local /root/.ssh/config
COPY meta/docker/ssh/known_hosts.local /root/.ssh/known_hosts
COPY meta/docker/ssh/keys/github /root/.ssh/keys/github
COPY meta/docker/TASK_V1.md /workspace/meta/docker/TASK_V1.md
COPY meta/docker/TASK_AGENTS_V1.md /workspace/meta/docker/TASK_AGENTS_V1.md
COPY meta/docker/FRICTION_V1.md /workspace/meta/docker/FRICTION_V1.md
COPY meta/docker/RESULT_V1.md /workspace/meta/docker/RESULT_V1.md
COPY meta/docker/run-task-v1.sh /usr/local/bin/run-task-v1
COPY meta/docker/run-opencode-task-v1.sh /usr/local/bin/run-opencode-task-v1
COPY meta/docker/run-opencode-agents-smoke-v1.sh /usr/local/bin/run-opencode-agents-smoke-v1
RUN chmod 700 /root/.ssh \
  && chmod 600 /root/.local/share/opencode/auth.json /root/.config/gh/hosts.yml /root/.ssh/config /root/.ssh/known_hosts /root/.ssh/keys/github /tmp/gh-token \
  && chmod 644 /root/.cache/opencode/models.json
RUN gh auth login --hostname github.com --with-token < /tmp/gh-token && rm -f /tmp/gh-token
RUN chmod 0755 /usr/local/bin/run-task-v1 /usr/local/bin/run-opencode-task-v1 /usr/local/bin/run-opencode-agents-smoke-v1

ENV CONSENSUS_REPO_URL="git@github.com:powerformer/consensus.git"
ENV COWORK_HOME="/workspace/.cowork"
ENV COWORK_CLONE_REPO_URL="git@github.com:powerformer/consensus.git"
ENV COWORK_CLONE_REPO_ALIAS="consensus"
ENV COWORK_INSTALL_URL="https://raw.githubusercontent.com/powerformer/cowork-cli/main/install.sh"

WORKDIR /workspace

CMD ["bash", "-lc", "opencode --version && opencode auth list && gh --version && printf 'repo_url=%s\ncowork_home=%s\ncowork_clone_repo_url=%s\ncowork_clone_repo_alias=%s\ninstall_url=%s\n' \"$CONSENSUS_REPO_URL\" \"$COWORK_HOME\" \"$COWORK_CLONE_REPO_URL\" \"$COWORK_CLONE_REPO_ALIAS\" \"$COWORK_INSTALL_URL\" && printf 'task_prompt=%s\n' '/workspace/meta/docker/TASK_V1.md'"]
