FROM archlinux/archlinux:base-devel
LABEL maintainer="Yu Tokunaga <tokunaga@agni.ninja>"

# Add Meta
ARG UID=1000
ARG GID=1000
ARG USERNAME=hoe
ARG LOCATE=JP
ARG COUNTRY=ja_JP
ARG ENCODE=UTF-8
ENV TZ=Asia/Tokyo

# Install git and create user
RUN pacman -Syu --needed --noconfirm git \
  && groupadd -g ${GID} ${USERNAME} \
  && useradd -u ${UID} -g ${GID} --create-home ${USERNAME} \
  && echo "${USERNAME} ALL=(ALL:ALL) NOPASSWD:ALL" > /etc/sudoers.d/${USERNAME}

USER ${USERNAME}

# Set environment variables
ENV HOME_PATH=/home/${USERNAME}
ENV XDG_CONFIG_HOME=${HOME_PATH}/.config
WORKDIR ${HOME_PATH}

# Install yay
RUN git clone https://aur.archlinux.org/yay.git \
  && cd yay \
  && makepkg -sri --needed --noconfirm \
  && cd .. \
  # Clean up
  && rm -rf yay \
  && sudo pacman -Scc --noconfirm

# Install essential tools
RUN yay -S --needed --noconfirm fish exa ripgrep procs neovim rustup fzf nodejs npm \
  # Install pnpm
  && sudo npm install -g pnpm \
  # Install fisherman
  && curl -Lo ${XDG_CONFIG_HOME}/fish/functions/fisher.fish --create-dirs https://git.io/fisher \
  # Install fish-theme
  && fish -c "fisher install oh-my-fish/theme-bobthefish" \
  # Install cd history plugin
  && fish -c "fisher install jethrokuan/z" \
  # Install Rust
  && rustup default nightly \
  # Install nvim configuration
  && git clone https://github.com/4hoe8pow/nvimenv ${XDG_CONFIG_HOME}/nvim \
  && rm -rf ${XDG_CONFIG_HOME}/nvim/.git \
  # Configure git aliases
  && git config --global alias.a add \
  && git config --global alias.c commit \
  && git config --global alias.p push \
  && git config --global alias.st status \
  && git config --global alias.gr "log --graph --date=short --decorate=short --pretty=format:'%Cgreen%h %Creset%cd %Cblue%cn %Cred%d %Creset%s'" \
  && git config --global alias.empty "commit --allow-empty -m 'Initial commit'" \
  && git config --global alias.ch checkout

# Command Aliases
ARG fishrc=${XDG_CONFIG_HOME}/fish/config.fish
RUN echo "# Shell Operations" >> ${fishrc} \
  && echo "alias v='nvim'" >> ${fishrc} \
  && echo "alias l='exa --color auto --icons'" >> ${fishrc} \
  && echo "alias la='exa -la --color auto --icons'" >> ${fishrc} \
  && echo "alias ll='exa -l --color auto --icons'" >> ${fishrc} \
  && echo "alias tree='exa -T -L 3 --color auto --icons'" >> ${fishrc} \
  && echo "alias grep='rg'" >> ${fishrc} \
  && echo "alias ps='procs'" >> ${fishrc} \
  && echo "# Git" >> ${fishrc} \
  && echo "alias g='git'" >> ${fishrc} \
  && echo "alias gst='git st'" >> ${fishrc} \
  && echo "alias ga='git a'" >> ${fishrc} \
  && echo "alias gc='git c'" >> ${fishrc} \
  && echo "alias gp='git p'" >> ${fishrc} \
  && echo "alias p='pnpm'" >> ${fishrc} \
  && fish -c "funcsave fish_config" \
  # Clean up
  && sudo yay -Scc --noconfirm

ENTRYPOINT [ "fish" ]
